use gtk4::glib;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TypewriterState {
    pub current_line: String,
    pub last_char: Option<char>,
    pub strike_progress: f32, // 1.0 (strike point) -> 0.0 (rest)
    pub return_progress: f32, // for carriage return
    pub is_returning: bool,
}

impl TypewriterState {
    pub fn new() -> Self {
        Self {
            current_line: String::new(),
            last_char: None,
            strike_progress: 0.0,
            return_progress: 0.0,
            is_returning: false,
        }
    }
}

pub fn create_typewriter_widget() -> (gtk4::DrawingArea, Rc<RefCell<TypewriterState>>) {
    let area = gtk4::DrawingArea::new();
    area.set_height_request(76);
    area.set_hexpand(true);
    area.add_css_class("typewriter-chassis");

    let state = Rc::new(RefCell::new(TypewriterState::new()));
    let state_draw = state.clone();

    area.set_draw_func(move |_, cr, width, height| {
        let st = state_draw.borrow();
        let w = width as f64;
        let h = height as f64;
        let center_x = w / 2.0;

        // 1. Daktilo Gövdesi (Chassis background)
        cr.set_source_rgb(0.11, 0.11, 0.13); // #1c1c21
        let _ = cr.paint();

        // Üst ve alt metal çerçeve çizgisi
        cr.set_source_rgb(0.24, 0.24, 0.28);
        cr.set_line_width(1.5);
        cr.move_to(0.0, 1.0);
        cr.line_to(w, 1.0);
        cr.move_to(0.0, h - 1.0);
        cr.line_to(w, h - 1.0);
        let _ = cr.stroke();

        // 2. Şaryo / Merdane (Platen Roller)
        let roller_y = 10.0;
        let roller_h = 24.0;
        cr.set_source_rgb(0.16, 0.16, 0.18);
        cr.rectangle(10.0, roller_y, w - 20.0, roller_h);
        let _ = cr.fill();

        // 3. Kağıt (Vintage Paper on Roller)
        let paper_w = (w * 0.75).min(700.0);
        let mut paper_x = center_x - (paper_w / 2.0);
        if st.is_returning {
            paper_x += (st.return_progress as f64) * 60.0;
        }

        cr.set_source_rgb(0.96, 0.94, 0.88); // #f5f0e1
        cr.rectangle(paper_x, roller_y - 4.0, paper_w, roller_h + 8.0);
        let _ = cr.fill();

        // Kağıt üstündeki metin (Son yazılan karakterler)
        cr.set_source_rgb(0.18, 0.16, 0.14);
        cr.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Bold);
        cr.set_font_size(15.0);

        let line_text = &st.current_line;
        // Metni vuruş merkezine göre hizala (son harf merkeze yakın olsun)
        let char_spacing = 9.5;
        let text_offset_x = center_x - (line_text.chars().count() as f64 * char_spacing);

        let mut curr_x = text_offset_x;
        for c in line_text.chars() {
            if curr_x >= paper_x && curr_x <= paper_x + paper_w - 10.0 {
                cr.move_to(curr_x, roller_y + 14.0);
                let _ = cr.show_text(&c.to_string());
            }
            curr_x += char_spacing;
        }

        // 4. Orta Tipbar Kılavuzu (Chrome Typebar Guide - V Şeklinde)
        let guide_y = roller_y + roller_h + 2.0;
        cr.set_source_rgb(0.65, 0.65, 0.70);
        cr.set_line_width(2.0);
        // Sol kol
        cr.move_to(center_x - 12.0, guide_y + 14.0);
        cr.line_to(center_x - 3.0, guide_y);
        // Sağ kol
        cr.move_to(center_x + 12.0, guide_y + 14.0);
        cr.line_to(center_x + 3.0, guide_y);
        let _ = cr.stroke();

        // Şerit Tutucu (Ribbon carrier kırmızı-siyah detay)
        cr.set_source_rgb(0.85, 0.25, 0.20);
        cr.rectangle(center_x - 5.0, guide_y - 2.0, 10.0, 3.0);
        let _ = cr.fill();

        // 5. Vuran Tipbar Çekici (Type Hammer Animation)
        if st.strike_progress > 0.05 {
            let progress = st.strike_progress as f64;
            let hammer_tip_y = guide_y - (progress * 14.0);
            let hammer_base_y = h + 10.0;

            // Metal çekiç kolu
            cr.set_source_rgba(0.75, 0.75, 0.82, 0.95);
            cr.set_line_width(2.5);
            cr.move_to(center_x, hammer_base_y);
            cr.line_to(center_x, hammer_tip_y);
            let _ = cr.stroke();

            // Çekiç başlığı (Metal slug)
            cr.set_source_rgb(0.88, 0.88, 0.92);
            cr.rectangle(center_x - 4.0, hammer_tip_y - 5.0, 8.0, 6.0);
            let _ = cr.fill();

            // Harf mürekkep patlaması (Stamp flash effect)
            if let Some(ch) = st.last_char {
                cr.set_source_rgba(0.9, 0.2, 0.15, progress * 0.9);
                cr.set_font_size(18.0);
                cr.move_to(center_x - 5.0, hammer_tip_y - 8.0);
                let _ = cr.show_text(&ch.to_string());
            }
        }
    });

    // 60fps tick callback for smooth animation decay
    let state_tick = state.clone();
    area.add_tick_callback(move |area, _| {
        let mut st = state_tick.borrow_mut();
        let mut needs_redraw = false;

        if st.strike_progress > 0.0 {
            st.strike_progress -= 0.15;
            if st.strike_progress < 0.0 {
                st.strike_progress = 0.0;
            }
            needs_redraw = true;
        }

        if st.is_returning {
            st.return_progress -= 0.12;
            if st.return_progress <= 0.0 {
                st.return_progress = 0.0;
                st.is_returning = false;
            }
            needs_redraw = true;
        }

        if needs_redraw {
            area.queue_draw();
        }

        glib::ControlFlow::Continue
    });

    (area, state)
}
