use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_window(app: &adw::Application) -> adw::ApplicationWindow {
    let sound_manager = Rc::new(
        crate::sound_manager::SoundManager::new().expect("Failed to initialize audio")
    );
    
    let overlay = gtk4::Overlay::new();
    
    let text_view = crate::editor::create_editor(sound_manager.clone());
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
    buffer.connect_changed(move |buf| {
        let (words, chars) = crate::editor::get_stats(buf);
        status_clone.set_label(&format!("Kelimeler: {} | Karakterler: {}", words, chars));
    });
    
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_box.append(&overlay);
    main_box.append(&status_bar);
    
    let header = adw::HeaderBar::new();
    header.add_css_class("flat");
    
    let title = adw::WindowTitle::new("Typwriter", "Daktilo Odaklanma Alanı");
    header.set_title_widget(Some(&title));
    
    // Aç Butonu (.txt, .md, .docx)
    let open_btn = gtk4::Button::from_icon_name("document-open-symbolic");
    open_btn.set_tooltip_text(Some("Belge Aç (.docx, .txt, .md)"));
    header.pack_start(&open_btn);
    
    // Kaydet Butonu (.txt, .docx, .md)
    let save_btn = gtk4::Button::from_icon_name("document-save-symbolic");
    save_btn.set_tooltip_text(Some("Belge Kaydet (.docx, .txt, .md)"));
    header.pack_start(&save_btn);

    // Dosya Açma Olayı
    let buffer_open = text_view.buffer();
    let title_clone = title.clone();
    open_btn.connect_clicked(move |btn| {
        let dialog = gtk4::FileDialog::new();
        dialog.set_title("Belge Aç (.docx, .txt, .md)");
        let parent = btn.root().and_downcast::<gtk4::Window>();
        let buffer_clone = buffer_open.clone();
        let title_ref = title_clone.clone();
        dialog.open(parent.as_ref(), None::<&gtk4::gio::Cancellable>, move |res| {
            if let Ok(file) = res {
                if let Some(path) = file.path() {
                    match crate::document::load_document(&path) {
                        Ok(content) => {
                            buffer_clone.set_text(&content);
                            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                                title_ref.set_subtitle(&format!("Dosya: {}", file_name));
                            }
                        }
                        Err(err) => {
                            eprintln!("Açma hatası: {}", err);
                        }
                    }
                }
            }
        });
    });

    // Dosya Kaydetme Olayı
    let buffer_save = text_view.buffer();
    let title_clone2 = title.clone();
    save_btn.connect_clicked(move |btn| {
        let dialog = gtk4::FileDialog::new();
        dialog.set_title("Belge Kaydet");
        dialog.set_initial_name(Some("daktilo_yazisi.docx"));
        let parent = btn.root().and_downcast::<gtk4::Window>();
        let buffer_clone = buffer_save.clone();
        let title_ref = title_clone2.clone();
        dialog.save(parent.as_ref(), None::<&gtk4::gio::Cancellable>, move |res| {
            if let Ok(file) = res {
                if let Some(path) = file.path() {
                    let text = buffer_clone.text(&buffer_clone.start_iter(), &buffer_clone.end_iter(), false);
                    match crate::document::save_document(&path, text.as_str()) {
                        Ok(_) => {
                            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                                title_ref.set_subtitle(&format!("Kaydedildi: {}", file_name));
                            }
                        }
                        Err(err) => {
                            eprintln!("Kaydetme hatası: {}", err);
                        }
                    }
                }
            }
        });
    });

    let atmospheres = crate::atmosphere::Atmosphere::all();
    let atm_labels: Vec<String> = atmospheres.iter()
        .map(|a| format!("{} {}", a.icon(), a.label()))
        .collect();
    let string_list = gtk4::StringList::new(&atm_labels.iter().map(|s| s.as_str()).collect::<Vec<_>>());
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
    
    let fullscreen_btn = gtk4::Button::from_icon_name("view-fullscreen-symbolic");
    fullscreen_btn.set_tooltip_text(Some("Tam Ekran (F11)"));
    header.pack_end(&fullscreen_btn);
    
    let toolbar_view = adw::ToolbarView::new();
    toolbar_view.add_top_bar(&header);
    toolbar_view.set_content(Some(&main_box));
    
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Typwriter")
        .default_width(900)
        .default_height(700)
        .content(&toolbar_view)
        .build();
    
    window.add_css_class("typewriter-window");
    window.add_css_class("atmosphere-silent");
    
    *window_ref.borrow_mut() = Some(window.clone());
    
    let win_clone = window.clone();
    fullscreen_btn.connect_clicked(move |_| {
        win_clone.set_fullscreened(!win_clone.is_fullscreen());
    });
    
    let key_controller = gtk4::EventControllerKey::new();
    let win_clone2 = window.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        if keyval == gtk4::gdk::Key::F11 {
            win_clone2.set_fullscreened(!win_clone2.is_fullscreen());
            return gtk4::glib::Propagation::Stop;
        }
        gtk4::glib::Propagation::Proceed
    });
    window.add_controller(key_controller);
    
    text_view.grab_focus();
    
    window
}
