use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_editor(
    sound_manager: Rc<crate::sound_manager::SoundManager>,
    typewriter_state: Rc<RefCell<crate::typewriter_view::TypewriterState>>,
) -> gtk4::TextView {
    let text_view = gtk4::TextView::new();
    text_view.set_wrap_mode(gtk4::WrapMode::Word);
    text_view.set_left_margin(60);
    text_view.set_right_margin(60);
    text_view.set_top_margin(40);
    text_view.set_bottom_margin(40);
    text_view.add_css_class("typewriter-paper");
    text_view.set_monospace(true);
    text_view.set_cursor_visible(true);
    text_view.set_vexpand(true);
    text_view.set_hexpand(true);

    // Key press event handler
    let key_controller = gtk4::EventControllerKey::new();
    let sm = sound_manager.clone();
    let tw = typewriter_state.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        match keyval {
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                sm.play_enter();
                let mut st = tw.borrow_mut();
                st.is_returning = true;
                st.return_progress = 1.0;
                st.strike_progress = 1.0;
                st.last_char = Some('⏎');
                st.current_line.clear();
            }
            gtk4::gdk::Key::space => {
                sm.play_space();
                let mut st = tw.borrow_mut();
                st.strike_progress = 1.0;
                st.last_char = Some(' ');
                st.current_line.push(' ');
                if st.current_line.len() > 50 {
                    let trim_idx = st.current_line.char_indices().nth(10).map(|(i, _)| i).unwrap_or(0);
                    st.current_line = st.current_line[trim_idx..].to_string();
                }
            }
            gtk4::gdk::Key::BackSpace => {
                sm.play_backspace();
                let mut st = tw.borrow_mut();
                st.current_line.pop();
                st.last_char = None;
            }
            _ => {
                if let Some(ch) = keyval.to_unicode() {
                    sm.play_key_click();
                    let mut st = tw.borrow_mut();
                    st.strike_progress = 1.0;
                    st.last_char = Some(ch);
                    st.current_line.push(ch);
                    if st.current_line.len() > 50 {
                        let trim_idx = st.current_line.char_indices().nth(10).map(|(i, _)| i).unwrap_or(0);
                        st.current_line = st.current_line[trim_idx..].to_string();
                    }
                }
            }
        }
        gtk4::glib::Propagation::Proceed
    });
    text_view.add_controller(key_controller);

    text_view
}

pub fn get_stats(buffer: &gtk4::TextBuffer) -> (usize, usize) {
    let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
    let text_str = text.as_str();
    let chars = text_str.chars().count();
    let words = text_str.split_whitespace().count();
    (words, chars)
}
