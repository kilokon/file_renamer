#[derive(Debug)]
struct DirFiles {
    f_path: String,
    f_name: String,
    f_extension: Option<String>,
}




#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
// #[derive(Default)]
pub struct RenamerApp {
    // path_ui_label: ,
    picked_path: String,
    bbx: bool,

    #[serde(skip)]
    files_in_dir: Vec<DirFiles>,
    // files_in_dir: Vec<std::fs::DirEntry>,
}

impl Default for RenamerApp {
    fn default() -> Self {
        Self {
            // path_ui_label: " ".into(),
            picked_path: "".into(),
            bbx: true,
            files_in_dir: Vec::new(),
        }
    }
}

impl RenamerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for RenamerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            files_in_dir,
            bbx,
            picked_path,
        } = self;
        egui::SidePanel::left("left Panel")
            .max_width(400.0)
            // .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("File Location");
                    ui.add(
                        egui::TextEdit::singleline(picked_path).hint_text("Paste Directory Path"),
                    );
                });
                if ui.button("Load Files").clicked() {
                    files_in_dir.clear();
                    let path = std::path::Path::new(picked_path);
                    if path.exists() {
                        let dir_content = path
                            .read_dir()
                            .expect("read_dir call failed")
                            .into_iter()
                            .filter_map(|f| f.ok())
                            .filter(|f| f.path().is_file())
                            .collect::<Vec<_>>();
                        for files in dir_content {
                            let mut _file = DirFiles{
                                f_name: String::from(files.file_name().to_str().unwrap()),
                                f_extension: Some(String::from(files.path().extension().unwrap().to_str().unwrap())),
                                f_path: String::from(files.path().to_str().unwrap()),
                            }; 
                            // println!("{:?}", &_file);
                            files_in_dir.push(_file);
                        }
                        // println!("{:?}", dir_content);
                    } else {
                        println!("Not a valid path!");
                    }
                }
                ui.separator();
                egui::CollapsingHeader::new("File Extensions").default_open(true).show(ui, |ui|{
                    ui.horizontal_wrapped(|ui| {
                        for i in files_in_dir.iter() {
                            let checkbox_text = i.f_extension.as_ref().unwrap();
                            ui.checkbox(bbx, checkbox_text);

                            // println!("{:?}", i);
                            // ui.checkbox(&mut true, i.f_extension.unwrap());
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("All the files");
            egui::ScrollArea::both().show(ui, |ui| {
                for i in files_in_dir {
                    // let filename = egui::RichText::i.path().file_name().unwrap().to_str();
                    // ui.label(text)
                    // let m = egui::WidgetText::RichText(egui::RichText::from(
                    //     i.path().file_name().unwrap().to_str().expect("msg"),
                    // ));
                    // ui.label(m);
                }
            });
        });

        egui::SidePanel::right("right panel")
            // .resizable(false)
            .max_width(400.0)
            // .default_width(500.00)
            // .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Replacements");
            });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
