use gtk4::prelude::*;
use libadwaita as adw;
use adw::prelude::*;
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
    let clamp = adw::Clamp::new();
    clamp.set_maximum_size(current_paper_format.borrow().width_pixels());
    clamp.set_tightening_threshold(current_paper_format.borrow().width_pixels());
    clamp.set_child(Some(&text_view));
    clamp.set_vexpand(true);
    clamp.set_margin_top(28);
    clamp.set_margin_bottom(28);
    clamp.set_margin_start(24);
    clamp.set_margin_end(24);
    clamp.add_css_class("typewriter-sheet-container");

    let scrolled = gtk4::ScrolledWindow::new();
    scrolled.add_css_class("typewriter-desk-area");
    scrolled.set_child(Some(&clamp));
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
    dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        let atm = crate::atmosphere::Atmosphere::all()[idx];
        sm_clone.set_atmosphere(atm);
        if let Some(win) = dd.root().and_downcast::<adw::ApplicationWindow>() {
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

    // --- AYARLAR BUTONU (AdwPreferencesWindow) ---
    let settings_btn = gtk4::Button::from_icon_name("open-menu-symbolic");
    settings_btn.set_tooltip_text(Some(strings.settings_tooltip));
    header.pack_end(&settings_btn);

    let tw_widget_ref = tw_widget.clone();
    let tw_state_ref = tw_state.clone();
    let sm_ref = sound_manager.clone();
    let pf_ref = current_paper_format.clone();
    let lang_ref = current_lang.clone();
    let clamp_ref = clamp.clone();
    let tv_ref = text_view.clone();
    let title_ref = title.clone();
    let open_btn_ref = open_btn.clone();
    let save_btn_ref = save_btn.clone();
    let save_as_ref = save_as_btn.clone();
    let fs_ref = fullscreen_btn.clone();
    let settings_btn_ref = settings_btn.clone();
    let status_lbl_ref = status_bar.clone();

    settings_btn.connect_clicked(move |btn| {
        let parent_win = btn.root().and_downcast::<adw::ApplicationWindow>();
        let current_language = *lang_ref.borrow();
        let s = get_strings(current_language);

        let prefs = adw::PreferencesWindow::builder()
            .modal(true)
            .title(s.settings_tooltip)
            .default_width(520)
            .default_height(420)
            .build();

        if let Some(win) = &parent_win {
            prefs.set_transient_for(Some(win));
        }

        let page = adw::PreferencesPage::new();
        let group = adw::PreferencesGroup::new();
        group.set_title(if current_language == Language::Turkish {
            "Görünüm ve Tercihler"
        } else {
            "Appearance & Preferences"
        });

        // 1. Daktilo Animasyonu
        let anim_row = adw::SwitchRow::new();
        anim_row.set_title(s.animation_switch);
        anim_row.set_subtitle(if current_language == Language::Turkish {
            "Alttaki mekanik daktilo çekiç animasyonu"
        } else {
            "Mechanical typewriter animation at the bottom"
        });
        anim_row.set_active(tw_widget_ref.is_visible());
        let tw_for_sw = tw_widget_ref.clone();
        anim_row.connect_active_notify(move |sw| {
            tw_for_sw.set_visible(sw.is_active());
        });
        group.add(&anim_row);

        // 2. Daktilo Sesleri
        let sound_row = adw::SwitchRow::new();
        sound_row.set_title(s.sound_switch);
        sound_row.set_subtitle(if current_language == Language::Turkish {
            "Mekanik tuş vuruşu ve satır sonu zili"
        } else {
            "Mechanical keystrokes and end-of-line bell"
        });
        sound_row.set_active(sm_ref.is_typing_enabled());
        let sm_for_sw = sm_ref.clone();
        sound_row.connect_active_notify(move |sw| {
            sm_for_sw.set_typing_enabled(sw.is_active());
        });
        group.add(&sound_row);

        // 3. Kağıt Formatı (A4 / Roman)
        let format_row = adw::ComboRow::new();
        format_row.set_title(s.paper_format_label);
        format_row.set_subtitle(if current_language == Language::Turkish {
            "Sayfa kenar boşlukları ve satır uzunluğu"
        } else {
            "Page margins and line length"
        });
        let format_items = [s.format_a4, s.format_novel];
        let format_model = gtk4::StringList::new(format_items.as_slice());
        format_row.set_model(Some(&format_model));
        let cur_fmt = *pf_ref.borrow();
        format_row.set_selected(if cur_fmt == crate::paper_format::PaperFormat::Novel { 1 } else { 0 });

        let pf_inner = pf_ref.clone();
        let tw_state_inner = tw_state_ref.clone();
        let tw_widget_inner = tw_widget_ref.clone();
        let clamp_inner = clamp_ref.clone();
        let tv_inner = tv_ref.clone();
        format_row.connect_selected_notify(move |cr| {
            let new_fmt = if cr.selected() == 1 {
                crate::paper_format::PaperFormat::Novel
            } else {
                crate::paper_format::PaperFormat::A4
            };
            *pf_inner.borrow_mut() = new_fmt;
            tw_state_inner.borrow_mut().paper_format = new_fmt;
            tw_widget_inner.queue_draw();
            clamp_inner.set_maximum_size(new_fmt.width_pixels());
            clamp_inner.set_tightening_threshold(new_fmt.width_pixels());
            tv_inner.set_width_request(new_fmt.width_pixels());
        });
        group.add(&format_row);

        // 4. Dil (Türkçe / English)
        let lang_row = adw::ComboRow::new();
        lang_row.set_title(s.language_label);
        let lang_items = ["Türkçe", "English"];
        let lang_model = gtk4::StringList::new(lang_items.as_slice());
        lang_row.set_model(Some(&lang_model));
        lang_row.set_selected(if current_language == Language::English { 1 } else { 0 });

        let lang_inner = lang_ref.clone();
        let title_inner = title_ref.clone();
        let open_inner = open_btn_ref.clone();
        let save_inner = save_btn_ref.clone();
        let save_as_inner = save_as_ref.clone();
        let fs_inner = fs_ref.clone();
        let settings_inner = settings_btn_ref.clone();
        let status_inner = status_lbl_ref.clone();
        let tv_for_stats = tv_ref.clone();

        lang_row.connect_selected_notify(move |cr| {
            let new_l = if cr.selected() == 1 {
                Language::English
            } else {
                Language::Turkish
            };
            *lang_inner.borrow_mut() = new_l;
            let ns = get_strings(new_l);

            title_inner.set_title(ns.app_title);
            open_inner.set_tooltip_text(Some(ns.open_tooltip));
            save_inner.set_tooltip_text(Some(ns.save_tooltip));
            save_as_inner.set_tooltip_text(Some(ns.save_as_tooltip));
            fs_inner.set_tooltip_text(Some(ns.fullscreen_tooltip));
            settings_inner.set_tooltip_text(Some(ns.settings_tooltip));

            let buf = tv_for_stats.buffer();
            let (words, chars) = crate::editor::get_stats(&buf);
            status_inner.set_label(&format!(
                "{}: {} | {}: {}",
                ns.words_label, words, ns.chars_label, chars
            ));
        });
        group.add(&lang_row);

        page.add(&group);
        prefs.add(&page);
        prefs.present();
    });

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

    // Tam ekran toggle
    fullscreen_btn.connect_clicked(move |btn| {
        if let Some(win) = btn.root().and_downcast::<adw::ApplicationWindow>() {
            win.set_fullscreened(!win.is_fullscreen());
        }
    });

    // Klavye kısayolları (F11: Tam Ekran, Ctrl+S: Kaydet, Ctrl+Shift+S: Farklı Kaydet, Ctrl+O: Aç)
    let key_controller = gtk4::EventControllerKey::new();
    let do_save_shortcut = do_direct_save.clone();
    let do_save_as_shortcut = do_save_as.clone();
    let open_btn_shortcut = open_btn.clone();

    key_controller.connect_key_pressed(move |ctrl, keyval, _, state| {
        let parent_win = ctrl.widget().and_downcast::<adw::ApplicationWindow>();
        let win_ref = parent_win.as_ref().map(|w| w.upcast_ref::<gtk4::Window>());

        if keyval == gtk4::gdk::Key::F11 {
            if let Some(win) = &parent_win {
                win.set_fullscreened(!win.is_fullscreen());
            }
            return gtk4::glib::Propagation::Stop;
        }

        let is_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
        let is_shift = state.contains(gtk4::gdk::ModifierType::SHIFT_MASK);

        if is_ctrl {
            if keyval == gtk4::gdk::Key::s || keyval == gtk4::gdk::Key::S {
                if is_shift {
                    do_save_as_shortcut(win_ref);
                } else {
                    do_save_shortcut(win_ref);
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
