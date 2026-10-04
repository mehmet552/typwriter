use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::i18n::{get_strings, Language};

pub fn build_window(app: &adw::Application) -> adw::ApplicationWindow {
    let current_lang = Rc::new(RefCell::new(Language::Turkish));
    let current_file_path: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));

    let current_paper_format = Rc::new(RefCell::new(crate::paper_format::PaperFormat::A4));

    let sound_manager = Rc::new(
        crate::sound_manager::SoundManager::new().expect("Failed to initialize audio"),
    );

    // Daktilo Animasyon Widget'ı
    let (tw_widget, tw_state) = crate::typewriter_view::create_typewriter_widget();

    let overlay = gtk4::Overlay::new();

    let text_view = crate::editor::create_editor(
        sound_manager.clone(),
        tw_state.clone(),
        current_paper_format.clone(),
    );
    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.set_child(Some(&text_view));
    scrolled.set_vexpand(true);

    let glow = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glow.add_css_class("glow-overlay");
    glow.set_can_target(false);
    glow.set_focusable(false);

    overlay.set_child(Some(&scrolled));
    overlay.add_overlay(&glow);

    let status_bar = gtk4::Label::new(Some("Kelimeler: 0 | Karakterler: 0"));
    status_bar.add_css_class("status-bar");
    status_bar.set_halign(gtk4::Align::Center);

    let buffer = text_view.buffer();
    let status_clone = status_bar.clone();
    let lang_for_stats = current_lang.clone();
    buffer.connect_changed(move |buf| {
        let (words, chars) = crate::editor::get_stats(buf);
        let strings = get_strings(*lang_for_stats.borrow());
        status_clone.set_label(&format!(
            "{}: {} | {}: {}",
            strings.words_label, words, strings.chars_label, chars
        ));
    });

    // Ana dikey düzen
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_box.append(&overlay);
    main_box.append(&tw_widget); // Alttaki daktilo animasyon mekanizması
    main_box.append(&status_bar);

    let header = adw::HeaderBar::new();
    header.add_css_class("flat");

    let strings = get_strings(*current_lang.borrow());
    let title = adw::WindowTitle::new(strings.app_title, strings.subtitle_default);
    header.set_title_widget(Some(&title));

    // 1. Belge Aç Butonu
    let open_btn = gtk4::Button::from_icon_name("document-open-symbolic");
    open_btn.set_tooltip_text(Some(strings.open_tooltip));
    header.pack_start(&open_btn);

    // 2. Doğrudan Kaydet Butonu (Mevcut dosyaya veya yeni dosyaya kaydeder)
    let save_btn = gtk4::Button::from_icon_name("document-save-symbolic");
    save_btn.set_tooltip_text(Some(strings.save_tooltip));
    header.pack_start(&save_btn);

    // 3. Farklı Kaydet Butonu
    let save_as_btn = gtk4::Button::from_icon_name("document-save-as-symbolic");
    save_as_btn.set_tooltip_text(Some(strings.save_as_tooltip));
    header.pack_start(&save_as_btn);

    // Ortak Dosya Kaydetme Fonksiyonu
    let do_save_as = {
        let text_view = text_view.clone();
        let title = title.clone();
        let current_file_path = current_file_path.clone();
        let current_lang = current_lang.clone();

        Rc::new(move |parent_win: Option<&gtk4::Window>| {
            let dialog = gtk4::FileDialog::new();
            let strings = get_strings(*current_lang.borrow());
            dialog.set_title(strings.save_dialog_title);

            let initial_name = current_file_path
                .borrow()
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "daktilo_yazisi.docx".to_string());

            dialog.set_initial_name(Some(&initial_name));

            let buffer = text_view.buffer();
            let title_clone = title.clone();
            let current_file_path = current_file_path.clone();
            let current_lang = current_lang.clone();

            dialog.save(parent_win, None::<&gtk4::gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                        if let Ok(()) = crate::document::save_document(&path, text.as_str()) {
                            let strings = get_strings(*current_lang.borrow());
                            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                title_clone.set_subtitle(&format!("{}: {}", strings.saved_prefix, name));
                            }
                            *current_file_path.borrow_mut() = Some(path);
                        }
                    }
                }
            });
        })
    };

    let do_direct_save = {
        let text_view = text_view.clone();
        let title = title.clone();
        let current_file_path = current_file_path.clone();
        let current_lang = current_lang.clone();
        let do_save_as_clone = do_save_as.clone();

        Rc::new(move |parent_win: Option<&gtk4::Window>| {
            let path_opt = current_file_path.borrow().clone();
            if let Some(path) = path_opt {
                // Mevcut açılmış dosyaya doğrudan kaydet!
                let buffer = text_view.buffer();
                let text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
                if let Ok(()) = crate::document::save_document(&path, text.as_str()) {
                    let strings = get_strings(*current_lang.borrow());
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        title.set_subtitle(&format!("{}: {}", strings.saved_prefix, name));
                    }
                }
            } else {
                // Dosya henüz belirlenmemişse Farklı Kaydet aç
                do_save_as_clone(parent_win);
            }
        })
    };

    // Aç Olayı
    let buffer_open = text_view.buffer();
    let title_open = title.clone();
    let current_file_open = current_file_path.clone();
    let lang_open = current_lang.clone();
    open_btn.connect_clicked(move |btn| {
        let dialog = gtk4::FileDialog::new();
        let strings = get_strings(*lang_open.borrow());
        dialog.set_title(strings.open_dialog_title);
        let parent = btn.root().and_downcast::<gtk4::Window>();
        let buffer_clone = buffer_open.clone();
        let title_clone = title_open.clone();
        let cur_file = current_file_open.clone();
        let lang = lang_open.clone();

        dialog.open(parent.as_ref(), None::<&gtk4::gio::Cancellable>, move |res| {
            if let Ok(file) = res {
                if let Some(path) = file.path() {
                    if let Ok(content) = crate::document::load_document(&path) {
                        buffer_clone.set_text(&content);
                        let strings = get_strings(*lang.borrow());
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            title_clone.set_subtitle(&format!("{}: {}", strings.opened_prefix, name));
                        }
                        *cur_file.borrow_mut() = Some(path);
                    }
                }
            }
        });
    });

    // Kaydet Olayı
    let do_save_click = do_direct_save.clone();
    save_btn.connect_clicked(move |btn| {
        let parent = btn.root().and_downcast::<gtk4::Window>();
        do_save_click(parent.as_ref());
    });

    // Farklı Kaydet Olayı
    let do_save_as_click = do_save_as.clone();
    save_as_btn.connect_clicked(move |btn| {
        let parent = btn.root().and_downcast::<gtk4::Window>();
        do_save_as_click(parent.as_ref());
    });

    // Atmosfer Seçici
    let atmospheres = crate::atmosphere::Atmosphere::all();
    let atm_labels: Vec<String> = atmospheres
        .iter()
        .map(|a| format!("{} {}", a.icon(), a.label()))
        .collect();
    let string_list =
        gtk4::StringList::new(&atm_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let dropdown = gtk4::DropDown::new(Some(string_list), gtk4::Expression::NONE);
    dropdown.set_selected(0);
    dropdown.add_css_class("atmosphere-selector");

    let sm_clone = sound_manager.clone();
    let window_ref: Rc<RefCell<Option<adw::ApplicationWindow>>> = Rc::new(RefCell::new(None));
    let win_ref_clone = window_ref.clone();
    dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        let atm = crate::atmosphere::Atmosphere::all()[idx];
        sm_clone.set_atmosphere(atm);
        if let Some(win) = win_ref_clone.borrow().as_ref() {
            for a in crate::atmosphere::Atmosphere::all() {
                win.remove_css_class(a.css_class());
            }
            win.add_css_class(atm.css_class());
        }
    });
    header.pack_end(&dropdown);

    // Tam Ekran Butonu
    let fullscreen_btn = gtk4::Button::from_icon_name("view-fullscreen-symbolic");
    fullscreen_btn.set_tooltip_text(Some(strings.fullscreen_tooltip));
    header.pack_end(&fullscreen_btn);

    // --- AYARLAR MENÜSÜ (Preferences / Settings Popover) ---
    let settings_btn = gtk4::MenuButton::new();
    settings_btn.set_icon_name("open-menu-symbolic");
    settings_btn.set_tooltip_text(Some(strings.settings_tooltip));

    let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    popover_box.set_margin_start(16);
    popover_box.set_margin_end(16);
    popover_box.set_margin_top(16);
    popover_box.set_margin_bottom(16);

    // 1. Daktilo Animasyonu Aç/Kapa Anahtarı
    let anim_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    let anim_label = gtk4::Label::new(Some(strings.animation_switch));
    anim_label.set_hexpand(true);
    anim_label.set_halign(gtk4::Align::Start);
    let anim_switch = gtk4::Switch::new();
    anim_switch.set_active(true);
    let tw_toggle = tw_widget.clone();
    anim_switch.connect_active_notify(move |sw| {
        tw_toggle.set_visible(sw.is_active());
    });
    anim_row.append(&anim_label);
    anim_row.append(&anim_switch);
    popover_box.append(&anim_row);

    // 2. Daktilo Tuş Sesleri Aç/Kapa Anahtarı
    let sound_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    let sound_label = gtk4::Label::new(Some(strings.sound_switch));
    sound_label.set_hexpand(true);
    sound_label.set_halign(gtk4::Align::Start);
    let sound_switch = gtk4::Switch::new();
    sound_switch.set_active(true);
    let sm_toggle = sound_manager.clone();
    sound_switch.connect_active_notify(move |sw| {
        sm_toggle.set_typing_enabled(sw.is_active());
    });
    sound_row.append(&sound_label);
    sound_row.append(&sound_switch);
    popover_box.append(&sound_row);

    // Ayırıcı çizgi
    popover_box.append(&gtk4::Separator::new(gtk4::Orientation::Horizontal));

    // 3. Dil Seçici (TR / EN)
    let lang_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    let lang_label = gtk4::Label::new(Some(strings.language_label));
    lang_label.set_hexpand(true);
    lang_label.set_halign(gtk4::Align::Start);

    let lang_dropdown = gtk4::DropDown::from_strings(&["Türkçe", "English"]);
    lang_dropdown.set_selected(0);

    let format_lbl_ref = gtk4::Label::new(Some(strings.paper_format_label));
    format_lbl_ref.set_hexpand(true);
    format_lbl_ref.set_halign(gtk4::Align::Start);

    let format_dropdown = gtk4::DropDown::from_strings(&[strings.format_a4, strings.format_novel]);
    format_dropdown.set_selected(0);

    let pf_state = current_paper_format.clone();
    let tv_for_format = text_view.clone();
    format_dropdown.connect_selected_notify(move |dd| {
        let new_format = if dd.selected() == 1 {
            crate::paper_format::PaperFormat::Novel
        } else {
            crate::paper_format::PaperFormat::A4
        };
        *pf_state.borrow_mut() = new_format;
        tv_for_format.set_width_request(new_format.width_pixels());
    });

    let format_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    format_row.append(&format_lbl_ref);
    format_row.append(&format_dropdown);

    // Dil değiştiğinde arayüz yazılarını güncelle
    let lang_state = current_lang.clone();
    let title_ref = title.clone();
    let open_btn_ref = open_btn.clone();
    let save_btn_ref = save_btn.clone();
    let save_as_ref = save_as_btn.clone();
    let fs_ref = fullscreen_btn.clone();
    let anim_lbl_ref = anim_label.clone();
    let sound_lbl_ref = sound_label.clone();
    let lang_lbl_ref = lang_label.clone();
    let format_lbl_update = format_lbl_ref.clone();
    let status_lbl_ref = status_bar.clone();
    let text_view_ref = text_view.clone();

    lang_dropdown.connect_selected_notify(move |dd| {
        let new_lang = if dd.selected() == 1 {
            Language::English
        } else {
            Language::Turkish
        };
        *lang_state.borrow_mut() = new_lang;
        let s = get_strings(new_lang);

        open_btn_ref.set_tooltip_text(Some(s.open_tooltip));
        save_btn_ref.set_tooltip_text(Some(s.save_tooltip));
        save_as_ref.set_tooltip_text(Some(s.save_as_tooltip));
        fs_ref.set_tooltip_text(Some(s.fullscreen_tooltip));
        anim_lbl_ref.set_text(s.animation_switch);
        sound_lbl_ref.set_text(s.sound_switch);
        lang_lbl_ref.set_text(s.language_label);
        format_lbl_update.set_text(s.paper_format_label);
        title_ref.set_title(s.app_title);

        let buf = text_view_ref.buffer();
        let (words, chars) = crate::editor::get_stats(&buf);
        status_lbl_ref.set_label(&format!(
            "{}: {} | {}: {}",
            s.words_label, words, s.chars_label, chars
        ));
    });

    lang_row.append(&lang_label);
    lang_row.append(&lang_dropdown);
    popover_box.append(&lang_row);
    popover_box.append(&format_row);

    let popover = gtk4::Popover::new();
    popover.set_child(Some(&popover_box));
    settings_btn.set_popover(Some(&popover));
    header.pack_end(&settings_btn);

    // ToolbarView
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&main_box));

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Typwriter")
        .default_width(920)
        .default_height(740)
        .content(&toolbar_view)
        .build();

    window.add_css_class("typewriter-window");
    window.add_css_class("atmosphere-silent");

    *window_ref.borrow_mut() = Some(window.clone());

    // Tam ekran toggle
    let win_clone = window.clone();
    fullscreen_btn.connect_clicked(move |_| {
        win_clone.set_fullscreened(!win_clone.is_fullscreen());
    });

    // Klavye kısayolları (F11: Tam Ekran, Ctrl+S: Kaydet, Ctrl+Shift+S: Farklı Kaydet, Ctrl+O: Aç)
    let key_controller = gtk4::EventControllerKey::new();
    let win_clone2 = window.clone();
    let do_save_shortcut = do_direct_save.clone();
    let do_save_as_shortcut = do_save_as.clone();
    let open_btn_shortcut = open_btn.clone();

    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        if keyval == gtk4::gdk::Key::F11 {
            win_clone2.set_fullscreened(!win_clone2.is_fullscreen());
            return gtk4::glib::Propagation::Stop;
        }

        let is_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let is_shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);

        if is_ctrl {
            if keyval == gtk4::gdk::Key::s || keyval == gtk4::gdk::Key::S {
                let win_ref = win_clone2.upcast_ref::<gtk4::Window>();
                if is_shift {
                    do_save_as_shortcut(Some(win_ref));
                } else {
                    do_save_shortcut(Some(win_ref));
                }
                return gtk4::glib::Propagation::Stop;
            } else if keyval == gtk4::gdk::Key::o || keyval == gtk4::gdk::Key::O {
                open_btn_shortcut.emit_clicked();
                return gtk4::glib::Propagation::Stop;
            }
        }

        gtk4::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);

    text_view.grab_focus();

    window
}
