use gtk4::glib;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

pub struct TypewriterState {
    pub current_line: String,
    pub last_char: Option<char>,
    pub strike_progress: f32,   // 1.0 (vurma anı) -> 0.0 (dinlenme)
    pub return_progress: f32,   // 1.0 (başa dönüyor) -> 0.0 (başta)
    pub is_returning: bool,
    pub carriage_steps: usize,  // Satırdaki karakter adımı
    pub active_key_index: Option<usize>, // 0..18 hangi çekiç kolu kalktı
    pub roller_roll_angle: f64, // Enter basıldığında silindirin dönme açısı
    pub bell_played_on_line: bool, // Satır sonu zili çaldı mı
}

impl TypewriterState {
    pub fn new() -> Self {
        Self {
            current_line: String::new(),
            last_char: None,
            strike_progress: 0.0,
            return_progress: 0.0,
            is_returning: false,
            carriage_steps: 0,
            active_key_index: None,
            roller_roll_angle: 0.0,
            bell_played_on_line: false,
        }
    }
}

pub fn create_typewriter_widget() -> (gtk4::DrawingArea, Rc<RefCell<TypewriterState>>) {
    let area = gtk4::DrawingArea::new();
    area.set_height_request(145);
    area.set_hexpand(true);
    area.add_css_class("typewriter-chassis");

    let state = Rc::new(RefCell::new(TypewriterState::new()));
    let state_draw = state.clone();

    area.set_draw_func(move |_, cr, width, height| {
        let st = state_draw.borrow();
        let w = width as f64;
        let h = height as f64;
        let center_x = w / 2.0;

        // ==========================================
        // 1. DAKTİLO METAL ŞASİ & ARKA PLAN (Chassis)
        // ==========================================
        // Vintage fırçalanmış koyu döküm demir gövde (#1a1a1f -> #111114)
        let pat = gtk4::cairo::LinearGradient::new(0.0, 0.0, 0.0, h);
        pat.add_color_stop_rgb(0.0, 0.16, 0.16, 0.20);
        pat.add_color_stop_rgb(0.4, 0.12, 0.12, 0.15);
        pat.add_color_stop_rgb(1.0, 0.08, 0.08, 0.10);
        cr.set_source(&pat).unwrap();
        let _ = cr.paint();

        // Üst metalik krom kenarlık ve perçinler
        cr.set_source_rgb(0.28, 0.28, 0.34);
        cr.set_line_width(2.0);
        cr.move_to(0.0, 1.0);
        cr.line_to(w, 1.0);
        let _ = cr.stroke();

        cr.set_source_rgb(0.45, 0.45, 0.52);
        cr.set_line_width(1.0);
        cr.move_to(0.0, 3.0);
        cr.line_to(w, 3.0);
        let _ = cr.stroke();

        // ==========================================
        // 2. HAREKETLİ ŞARYO & MERDANE (Carriage & Platen)
        // ==========================================
        // Şaryo hareketi: Her tuş basışında şaryo sola kayar (adım: 10px)
        // Enter basıldığında return_progress boyunca yumuşakça sağa (başa) kayar
        let step_offset = (st.carriage_steps as f64 * 10.0).min(center_x - 60.0);
        let carriage_shift = if st.is_returning {
            step_offset * (1.0 - st.return_progress as f64)
        } else {
            step_offset
        };

        let platen_y = 12.0;
        let platen_h = 32.0;
        let carriage_x = center_x - carriage_shift;

        // Şaryo arka rayı (Gümüş metal ray)
        cr.set_source_rgb(0.22, 0.22, 0.26);
        cr.rectangle(15.0, platen_y + 4.0, w - 30.0, platen_h - 8.0);
        let _ = cr.fill();

        // Merdane kauçuk silindiri (Platen Roller - döküm kauçuk silindir)
        let roller_w = (w * 0.85).max(500.0);
        let roller_x = carriage_x - (roller_w / 2.0);

        let roller_grad = gtk4::cairo::LinearGradient::new(0.0, platen_y, 0.0, platen_y + platen_h);
        roller_grad.add_color_stop_rgb(0.0, 0.12, 0.12, 0.14);
        roller_grad.add_color_stop_rgb(0.3, 0.26, 0.26, 0.30); // ışık yansıması
        roller_grad.add_color_stop_rgb(0.7, 0.14, 0.14, 0.16);
        roller_grad.add_color_stop_rgb(1.0, 0.08, 0.08, 0.10);
        cr.set_source(&roller_grad).unwrap();
        cr.rectangle(roller_x, platen_y, roller_w, platen_h);
        let _ = cr.fill();

        // Merdane sol ve sağ çevirme topuzları (Platen Knobs - tırtıklı gümüş)
        cr.set_source_rgb(0.55, 0.55, 0.62);
        // Sol topuz
        cr.rectangle(roller_x - 12.0, platen_y + 4.0, 12.0, platen_h - 8.0);
        // Sağ topuz
        cr.rectangle(roller_x + roller_w, platen_y + 4.0, 12.0, platen_h - 8.0);
        let _ = cr.fill();

        // Sol satır atlama ve şaryo başa kaydırma kolu (Carriage Return Lever)
        let lever_angle = if st.is_returning { 0.35 } else { 0.0 };
        cr.save().unwrap();
        cr.translate(roller_x - 6.0, platen_y + 8.0);
        cr.rotate(-lever_angle);
        cr.set_source_rgb(0.72, 0.72, 0.80); // Krom parlak kol
        cr.set_line_width(4.0);
        cr.move_to(0.0, 0.0);
        cr.line_to(-28.0, -18.0);
        cr.line_to(-42.0, -12.0);
        let _ = cr.stroke();
        // Kol ucu topuzu
        cr.set_source_rgb(0.85, 0.85, 0.92);
        cr.arc(-42.0, -12.0, 4.5, 0.0, 2.0 * PI);
        let _ = cr.fill();
        cr.restore().unwrap();

        // ==========================================
        // 3. MERDANE ÜZERİNDEKİ KAĞIT (Paper on Platen)
        // ==========================================
        let paper_w = 460.0;
        let paper_x = carriage_x - 60.0; // Yazının başlayacağı yer
        let paper_y = platen_y - 10.0;
        let paper_h = platen_h + 16.0;

        // Kağıt gölgesi
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.35);
        cr.rectangle(paper_x - 2.0, paper_y + 2.0, paper_w + 4.0, paper_h);
        let _ = cr.fill();

        // Antik daktilo kağıdı (#fbf9f1)
        cr.set_source_rgb(0.97, 0.95, 0.90);
        cr.rectangle(paper_x, paper_y, paper_w, paper_h);
        let _ = cr.fill();

        // Kağıt kenar çizgisi
        cr.set_source_rgb(0.82, 0.78, 0.70);
        cr.set_line_width(1.0);
        cr.rectangle(paper_x, paper_y, paper_w, paper_h);
        let _ = cr.stroke();

        // Kağıt baskı çıtası (Paper Bail - merdane üstündeki şeffaf/metal cetvel)
        cr.set_source_rgba(0.70, 0.70, 0.78, 0.65);
        cr.set_line_width(2.0);
        cr.move_to(roller_x, platen_y + 20.0);
        cr.line_to(roller_x + roller_w, platen_y + 20.0);
        let _ = cr.stroke();

        // Küçük kağıt tutucu kauçuk makaralar
        cr.set_source_rgb(0.20, 0.20, 0.24);
        cr.rectangle(center_x - 140.0, platen_y + 17.0, 8.0, 6.0);
        cr.rectangle(center_x + 140.0, platen_y + 17.0, 8.0, 6.0);
        let _ = cr.fill();

        // ==========================================
        // 4. KAĞITTAKİ METİN (Typed Characters)
        // ==========================================
        cr.save().unwrap();
        // Kağıdın dışına taşan yazıyı kırp
        cr.rectangle(paper_x + 4.0, paper_y, paper_w - 8.0, paper_h);
        cr.clip();

        cr.set_source_rgb(0.14, 0.12, 0.10);
        cr.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Bold);
        cr.set_font_size(16.0);

        let char_spacing = 10.0;
        let mut text_draw_x = paper_x + 16.0;
        let text_draw_y = platen_y + 18.0;

        for ch in st.current_line.chars() {
            cr.move_to(text_draw_x, text_draw_y);
            let _ = cr.show_text(&ch.to_string());
            text_draw_x += char_spacing;
        }
        cr.restore().unwrap();

        // ==========================================
        // 5. ÇEKİÇ SEPETİ (Typebar Basket & Metal Arms)
        // ==========================================
        // Daktilonun iç kısmındaki dairesel kavisli çekiç kolları (19 adet radyal kol)
        let basket_center_y = h + 45.0;
        let basket_radius = 95.0;
        let basket_inner_radius = 55.0;

        cr.set_source_rgb(0.12, 0.12, 0.15);
        cr.arc(center_x, basket_center_y, basket_radius + 10.0, PI, 2.0 * PI);
        let _ = cr.fill();

        let num_bars = 19;
        let start_angle = PI + 0.35;
        let end_angle = (2.0 * PI) - 0.35;
        let angle_step = (end_angle - start_angle) / (num_bars as f64 - 1.0);

        for i in 0..num_bars {
            let angle = start_angle + (i as f64 * angle_step);
            let is_striking_bar = st.active_key_index == Some(i) && st.strike_progress > 0.05;

            let bx1 = center_x + basket_radius * angle.cos();
            let by1 = basket_center_y + basket_radius * angle.sin();

            if is_striking_bar {
                // Bu kol vuruyor! (Yukarı fırlama animasyonu)
                let p = st.strike_progress as f64;
                let target_x = center_x;
                let target_y = platen_y + 12.0;

                let cur_x = bx1 + (target_x - bx1) * p;
                let cur_y = by1 + (target_y - by1) * p;

                // Parlak çelik kol
                cr.set_source_rgba(0.85, 0.85, 0.95, 0.98);
                cr.set_line_width(3.2);
                cr.move_to(center_x, h);
                cr.line_to(cur_x, cur_y);
                let _ = cr.stroke();

                // Harf kafası (Slug)
                cr.set_source_rgb(0.92, 0.92, 0.98);
                cr.rectangle(cur_x - 5.0, cur_y - 6.0, 10.0, 8.0);
                let _ = cr.fill();
            } else {
                // Dinlenmedeki kollar
                let bx2 = center_x + basket_inner_radius * angle.cos();
                let by2 = basket_center_y + basket_inner_radius * angle.sin();

                cr.set_source_rgba(0.38, 0.38, 0.44, 0.65);
                cr.set_line_width(1.8);
                cr.move_to(bx1, by1);
                cr.line_to(bx2, by2);
                let _ = cr.stroke();

                // Kol ucu çekiç kafası
                cr.set_source_rgba(0.55, 0.55, 0.62, 0.85);
                cr.arc(bx2, by2, 2.8, 0.0, 2.0 * PI);
                let _ = cr.fill();
            }
        }

        // ==========================================
        // 6. ŞERİT TAŞIYICI & V KILAVUZ (Ribbon Vibrator Guide)
        // ==========================================
        let guide_center_y = platen_y + 16.0;

        // Şerit (Siyah/Kırmızı Daktilo Şeridi)
        // Vurma anında şerit yukarı kalkar
        let ribbon_lift = if st.strike_progress > 0.05 { (st.strike_progress as f64) * 6.0 } else { 0.0 };
        let ribbon_y = guide_center_y + 8.0 - ribbon_lift;

        // Üst siyah şerit
        cr.set_source_rgb(0.12, 0.12, 0.14);
        cr.rectangle(center_x - 30.0, ribbon_y, 60.0, 4.0);
        let _ = cr.fill();
        // Alt kırmızı şerit
        cr.set_source_rgb(0.75, 0.18, 0.14);
        cr.rectangle(center_x - 30.0, ribbon_y + 4.0, 60.0, 4.0);
        let _ = cr.fill();

        // Krom V kılavuz çatalı (Chrome Type Guide)
        cr.set_source_rgb(0.80, 0.80, 0.88);
        cr.set_line_width(2.5);
        cr.move_to(center_x - 16.0, guide_center_y + 22.0);
        cr.line_to(center_x - 3.5, guide_center_y + 2.0);
        cr.line_to(center_x - 3.5, guide_center_y - 2.0);

        cr.move_to(center_x + 16.0, guide_center_y + 22.0);
        cr.line_to(center_x + 3.5, guide_center_y + 2.0);
        cr.line_to(center_x + 3.5, guide_center_y - 2.0);
        let _ = cr.stroke();

        // Kılavuz merkez yuvası
        cr.set_source_rgb(0.40, 0.40, 0.48);
        cr.rectangle(center_x - 8.0, guide_center_y + 20.0, 16.0, 6.0);
        let _ = cr.fill();

        // ==========================================
        // 7. VURUŞ ANINDA HARF DAMGASI (Ink Stamp Impact Flash)
        // ==========================================
        if st.strike_progress > 0.1 {
            let p = st.strike_progress as f64;
            if let Some(ch) = st.last_char {
                cr.save().unwrap();
                cr.set_source_rgba(0.88, 0.22, 0.16, p * 0.95); // Canlı şerit mürekkebi
                cr.select_font_face("monospace", gtk4::cairo::FontSlant::Normal, gtk4::cairo::FontWeight::Bold);
                cr.set_font_size(20.0);
                cr.move_to(center_x - 6.0, platen_y + 14.0);
                let _ = cr.show_text(&ch.to_string());

                // Darbe ışıması
                cr.set_source_rgba(1.0, 0.85, 0.5, p * 0.4);
                cr.arc(center_x, platen_y + 8.0, 12.0 * p, 0.0, 2.0 * PI);
                let _ = cr.fill();
                cr.restore().unwrap();
            }
        }

        // ==========================================
        // 8. ALT PANEL: VİNTAGE TUŞ TAKIMI ÇERÇEVESİ (Mini Keyboard Rim)
        // ==========================================
        let keys_y = h - 22.0;
        cr.set_source_rgb(0.09, 0.09, 0.11);
        cr.rectangle(0.0, keys_y, w, 22.0);
        let _ = cr.fill();

        cr.set_source_rgb(0.25, 0.25, 0.30);
        cr.set_line_width(1.0);
        cr.move_to(0.0, keys_y);
        cr.line_to(w, keys_y);
        let _ = cr.stroke();

        // Temsili dairesel daktilo tuşları
        let num_keys = 15;
        let key_spacing = 38.0;
        let start_key_x = center_x - ((num_keys as f64 - 1.0) * key_spacing / 2.0);

        for k in 0..num_keys {
            let kx = start_key_x + (k as f64 * key_spacing);
            if kx > 20.0 && kx < w - 20.0 {
                let is_pressed = st.strike_progress > 0.3 && (k == (st.active_key_index.unwrap_or(7) % num_keys));
                let ky = if is_pressed { keys_y + 13.0 } else { keys_y + 11.0 };

                // Krom çember
                cr.set_source_rgb(0.65, 0.65, 0.72);
                cr.arc(kx, ky, 8.5, 0.0, 2.0 * PI);
                let _ = cr.fill();

                // Siyah tuş içi
                cr.set_source_rgb(0.12, 0.12, 0.15);
                cr.arc(kx, ky, 7.0, 0.0, 2.0 * PI);
                let _ = cr.fill();

                // Tuş içi harf noktası
                cr.set_source_rgb(0.85, 0.82, 0.75);
                cr.arc(kx, ky, 1.8, 0.0, 2.0 * PI);
                let _ = cr.fill();
            }
        }
    });

    // 60 FPS Yumuşak Fizik ve Animasyon Döngüsü
    let state_tick = state.clone();
    area.add_tick_callback(move |area, _| {
        let mut st = state_tick.borrow_mut();
        let mut needs_redraw = false;

        // Çekiç vuruşunun geri çekilme hareketi (Easing)
        if st.strike_progress > 0.0 {
            st.strike_progress -= 0.14; // Hızlı vuruş ve yaylanarak geri dönüş
            if st.strike_progress <= 0.0 {
                st.strike_progress = 0.0;
                st.active_key_index = None;
            }
            needs_redraw = true;
        }

        // Şaryo başa dönme kayması (Carriage Return)
        if st.is_returning {
            st.return_progress -= 0.035; // ~1.2 saniyede başa dönüş (return.wav sesiyle senkronize!)
            st.roller_roll_angle += 0.2;
            if st.return_progress <= 0.0 {
                st.return_progress = 0.0;
                st.is_returning = false;
                st.carriage_steps = 0;
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
