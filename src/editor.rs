use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_editor(
    sound_manager: Rc<crate::sound_manager::SoundManager>,
    typewriter_state: Rc<RefCell<crate::typewriter_view::TypewriterState>>,
    paper_format: Rc<RefCell<crate::paper_format::PaperFormat>>,
) -> gtk4::TextView {
    let text_view = gtk4::TextView::new();
    text_view.set_wrap_mode(gtk4::WrapMode::Char);
    text_view.set_left_margin(55);
    text_view.set_right_margin(55);
    text_view.set_top_margin(45);
    text_view.set_bottom_margin(45);
    text_view.add_css_class("typewriter-paper");
    text_view.add_css_class("typewriter-sheet");
    text_view.set_monospace(true);
    text_view.set_cursor_visible(true);
    text_view.set_vexpand(true);
    text_view.set_halign(gtk4::Align::Center);
    text_view.set_width_request(paper_format.borrow().width_pixels());

    // Tuş basma olay denetleyicisi
    let key_controller = gtk4::EventControllerKey::new();
    let sm = sound_manager.clone();
    let tw = typewriter_state.clone();
    let tv_clone = text_view.clone();
    let pf_clone = paper_format.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        let is_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let is_alt = state.contains(gtk4::gdk::ModifierType::ALT_MASK);

        // Kısayollara (Ctrl+S, Ctrl+O, Ctrl+Shift+S vb.) doğrudan izin ver
        if is_ctrl || is_alt {
            return gtk4::glib::Propagation::Proceed;
        }

        let capacity = pf_clone.borrow().line_capacity();

        // Mevcut imlecin satırdaki karakter konumu (satır başından kaçıncı karakter)
        let buf = tv_clone.buffer();
        let cur_line_col = if let Some(mark) = buf.mark("insert") {
            let iter = buf.iter_at_mark(&mark);
            iter.line_offset() as usize
        } else {
            0
        };
        let has_selection = buf.has_selection();

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
                // Enter basıldığında alt satıra geçilir ve yazma kilidi açılır
                gtk4::glib::Propagation::Proceed
            }
            gtk4::gdk::Key::BackSpace => {
                sm.play_backspace();
                let mut st = tw.borrow_mut();
                st.current_line.pop();
                st.last_char = None;
                if st.carriage_steps > 0 {
                    st.carriage_steps -= 1;
                }
                if cur_line_col <= capacity {
                    st.bell_played_on_line = false;
                }
                gtk4::glib::Propagation::Proceed
            }
            gtk4::gdk::Key::space => {
                // Tik sesinden sonra (satır kapasitesine ulaşıldığında):
                // KİLİTLE! Otomatik alt satıra geçmeden yazmayı durdur. Enter şart!
                if !has_selection && cur_line_col >= capacity {
                    return gtk4::glib::Propagation::Stop;
                }

                sm.play_space();

                // Eğer bu boşluk satırın son izin verilen karakteriyse tik sesi (daktilo zili) çalar
                if cur_line_col + 1 >= capacity {
                    sm.play_bell();
                }

                let mut st = tw.borrow_mut();
                st.strike_progress = 0.4;
                st.last_char = Some(' ');
                st.carriage_steps = cur_line_col + 1;
                st.active_key_index = Some(9);
                st.current_line.push(' ');

                if st.current_line.len() > 45 {
                    let trim_idx = st.current_line.char_indices().nth(8).map(|(i, _)| i).unwrap_or(0);
                    st.current_line = st.current_line[trim_idx..].to_string();
                }

                gtk4::glib::Propagation::Proceed
            }
            _ => {
                if let Some(ch) = keyval.to_unicode() {
                    // Tik sesinden sonra (satır kapasitesine ulaşıldığında):
                    // KİLİTLE! Enter basıp alt satıra inmeden kesinlikle daha fazla yazılamaz!
                    if !has_selection && cur_line_col >= capacity {
                        return gtk4::glib::Propagation::Stop;
                    }

                    sm.play_key_click();

                    // Eğer bu harf satırın son karakteriyse, daktilo uyarı zili (tik sesi) çalar!
                    if cur_line_col + 1 >= capacity {
                        sm.play_bell();
                    }

                    let mut st = tw.borrow_mut();
                    st.carriage_steps = cur_line_col + 1;

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

                    gtk4::glib::Propagation::Proceed
                } else {
                    // Yön tuşları, Page Up/Down, Home, End gibi gezinme tuşlarına her zaman izin ver
                    gtk4::glib::Propagation::Proceed
                }
            }
        }
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
