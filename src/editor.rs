use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_editor(
    sound_manager: Rc<crate::sound_manager::SoundManager>,
    typewriter_state: Rc<RefCell<crate::typewriter_view::TypewriterState>>,
    paper_format: Rc<RefCell<crate::paper_format::PaperFormat>>,
) -> gtk4::TextView {
    let text_view = gtk4::TextView::new();
    text_view.set_wrap_mode(gtk4::WrapMode::Word);
    text_view.set_left_margin(50);
    text_view.set_right_margin(50);
    text_view.set_top_margin(40);
    text_view.set_bottom_margin(40);
    text_view.add_css_class("typewriter-paper");
    text_view.add_css_class("typewriter-sheet");
    text_view.set_monospace(true);
    text_view.set_cursor_visible(true);
    text_view.set_vexpand(true);
    text_view.set_halign(gtk4::Align::Center);
    text_view.set_width_request(paper_format.borrow().width_pixels());

    // Key press event handler
    let key_controller = gtk4::EventControllerKey::new();
    let sm = sound_manager.clone();
    let tw = typewriter_state.clone();
    let tv_clone = text_view.clone();
    let pf_clone = paper_format.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        // Mevcut imlecin satırdaki sütun konumunu al
        let buf = tv_clone.buffer();
        let cur_col = if let Some(mark) = buf.mark("insert") {
            let iter = buf.iter_at_mark(&mark);
            iter.line_offset() as usize
        } else {
            0
        };

        let margin_col = pf_clone.borrow().line_margin_cols();

        match keyval {
            gtk4::gdk::Key::Return | gtk4::gdk::Key::KP_Enter => {
                sm.play_enter();
                let mut st = tw.borrow_mut();
                st.is_returning = true;
                st.return_progress = 1.0;
                st.strike_progress = 0.0;
                st.last_char = None;
                st.active_key_index = None;
                st.carriage_steps = 0;
                st.bell_played_on_line = false;
                st.current_line.clear();
            }
            gtk4::gdk::Key::space => {
                sm.play_space();
                let mut st = tw.borrow_mut();
                st.strike_progress = 0.4;
                st.last_char = Some(' ');
                st.carriage_steps += 1;
                st.active_key_index = Some(9); // Ortadaki boşluk mekanizması
                st.current_line.push(' ');

                // Kağıt formatının satır sonu sınırına gelince daktilo zili / tik sesi çal
                let effective_col = cur_col.max(st.carriage_steps);
                if effective_col >= margin_col && !st.bell_played_on_line {
                    sm.play_bell();
                    st.bell_played_on_line = true;
                }

                if st.current_line.len() > 45 {
                    let trim_idx = st.current_line.char_indices().nth(8).map(|(i, _)| i).unwrap_or(0);
                    st.current_line = st.current_line[trim_idx..].to_string();
                }
            }
            gtk4::gdk::Key::BackSpace => {
                sm.play_backspace();
                let mut st = tw.borrow_mut();
                st.current_line.pop();
                st.last_char = None;
                if st.carriage_steps > 0 {
                    st.carriage_steps -= 1;
                }
                if cur_col < margin_col.saturating_sub(5) {
                    st.bell_played_on_line = false;
                }
            }
            _ => {
                if let Some(ch) = keyval.to_unicode() {
                    let mut st = tw.borrow_mut();
                    st.carriage_steps += 1;

                    // Kağıt formatının satır sonuna gelince (A4: 70, Roman: 52) daktilo zili / tik sesi çal
                    let effective_col = cur_col.max(st.carriage_steps);
                    if effective_col >= margin_col && !st.bell_played_on_line {
                        sm.play_bell();
                        st.bell_played_on_line = true;
                    }

                    sm.play_key_click();

                    // Klavyedeki harfe göre daktilo sepetindeki ilgili çekiç kolunu seç
                    let bar_idx = match ch.to_ascii_uppercase() {
                        'Q' | 'A' | 'Z' => 1,
                        'W' | 'S' | 'X' => 3,
                        'E' | 'D' | 'C' => 5,
                        'R' | 'F' | 'V' => 7,
                        'T' | 'G' | 'B' => 9,
                        'Y' | 'H' | 'N' => 10,
                        'U' | 'J' | 'M' => 12,
                        'I' | 'K' => 14,
                        'O' | 'L' => 16,
                        'P' => 17,
                        _ => ch as usize % 18,
                    };

                    st.strike_progress = 1.0;
                    st.active_key_index = Some(bar_idx);
                    st.last_char = Some(ch);
                    st.current_line.push(ch);

                    if st.current_line.len() > 45 {
                        let trim_idx = st.current_line.char_indices().nth(8).map(|(i, _)| i).unwrap_or(0);
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
