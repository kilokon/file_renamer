// use std::ffi::OsString;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Debug)]
enum ContextDirError {
    NotValidPathError,
}

impl std::fmt::Display for ContextDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not a valid path")
    }
}

#[derive(Debug, Clone)]
struct CurrentFileContext {
    f_path: PathBuf,
}

impl PartialEq for CurrentFileContext {
    fn eq(&self, other: &Self) -> bool {
        self.f_path.extension().unwrap() == other.f_path.extension().unwrap()
    }
}

impl CurrentFileContext {
    fn get_file_name(&self) -> Option<String> {
        self.f_path
            .file_name()
            .map(|x| x.to_str().unwrap().to_string())
    }

    fn get_file_extension(&self) -> Option<String> {
        self.f_path
            .extension()
            .as_ref()
            .map(|x| x.to_str().unwrap().to_string())
    }
}

#[derive(Debug)]
struct CurrentDirContext {
    all_files: Vec<CurrentFileContext>,
}

impl CurrentDirContext {
    fn get_all_file_names(&self) -> Vec<String> {
        self.all_files
            .into_iter()
            .map(|xx| xx.get_file_name().unwrap())
            .collect::<Vec<_>>()
    }
}

// impl std::iter::Iterator for CurrentDirContext {
//     type Item = CurrentFileContext;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut index: usize = 0;
//         while index < self.all_files.len() - 1 {
//             let prev = self.all_files.get(index).cloned();
//             index += 1;
//             return prev;
//         }
//         None
//     }
// }

fn get_all_paths(path: &Path) -> Option<CurrentDirContext> {
    // let mut files_in_dir: Vec<CurrentFileContext> = Vec::new();
    if path.exists() {
        let files_in_dir: CurrentDirContext = CurrentDirContext {
            all_files: path
                .read_dir()
                .expect("read_dir call failed")
                .into_iter()
                .filter_map(|f| f.ok())
                .filter(|f| f.path().is_file())
                .map(|x| CurrentFileContext { f_path: x.path() })
                .collect::<Vec<_>>(),
        };
        return Some(files_in_dir);
    }
    None
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
// #[derive(Default)]
pub struct RenamerApp {
    // path_ui_label: ,
    user_given_path: String,
    bbx: bool,

    #[serde(skip)]
    warning_text: String,
    #[serde(skip)]
    file_name_list: Vec<String>,
    #[serde(skip)]
    file_extensions: Vec<String>,
    #[serde(skip)]
    files_in_dir: Vec<CurrentFileContext>,
}

impl Default for RenamerApp {
    fn default() -> Self {
        Self {
            warning_text: "".into(),
            user_given_path: "".into(),
            files_in_dir: Vec::new(),
            bbx: true,
            file_name_list: Vec::new(),
            file_extensions: Vec::new(),
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
            warning_text,
            user_given_path,
            files_in_dir,
            bbx,
            file_name_list,
            file_extensions,
        } = self;
        file_extensions.clear();
        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| ui.label(warning_text.as_str()));
        egui::SidePanel::left("left Panel")
            .max_width(400.0)
            // .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label("File Location");
                    ui.add(
                        egui::TextEdit::singleline(user_given_path)
                            .hint_text("Paste Directory Path"),
                    );
                });
                ui.horizontal(|ui| {
                    if ui.button("Load Files").clicked() {
                        files_in_dir.clear();
                        warning_text.clear();
                        let user_given_path_p = std::path::Path::new(user_given_path);
                        if let Some(m) = get_all_paths(user_given_path_p) {
                            *files_in_dir = m.all_files;
                            *file_name_list = m
                                .all_files
                                .into_iter()
                                .map(|xx| xx.get_file_name().unwrap())
                                .collect::<Vec<_>>()
                        } else {
                            *warning_text = "Invalid path".to_string();
                            // println!("Bad file path")
                        }
                    }
                    //     if user_given_path_p.exists() {
                    //         *files_in_dir = user_given_path_p
                    //             .read_dir()
                    //             .expect("read_dir call failed")
                    //             .into_iter()
                    //             .map(|xx| xx.unwrap())
                    //             .map(|xx| CurrentFileContext { f_path: xx.path() })
                    //             .collect::<Vec<_>>();
                    //         *file_name_list = user_given_path_p
                    //             .read_dir()
                    //             .expect("read_dir call failed")
                    //             .into_iter()
                    //             .map(|xx| xx.unwrap())
                    //             .map(|xx| xx.file_name().into_string().ok().unwrap())
                    //             .collect::<Vec<_>>();
                    //     }
                    //     let m = get_all_paths(user_given_path_p).expect("msg");
                    // } else {
                    //     println!("Bad path")
                    // }
                    if ui.button("Clear").clicked() {
                        files_in_dir.clear();
                        file_extensions.clear();
                        user_given_path.clear();
                    }
                });
                ui.separator();
                egui::CollapsingHeader::new("Fiter Extensions")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            file_extensions.dedup();
                            for i in file_extensions {
                                ui.checkbox(bbx, i.as_str());
                            }
                        });
                    });
                egui::CollapsingHeader::new("File Padding")
                    .default_open(true)
                    .show(ui, |ui| {});
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("All the files");
            egui::ScrollArea::both().show(ui, |ui| {
                for file_name in file_name_list.iter() {
                    // let filename_list = files.f_name.as_str();
                    ui.label(file_name.as_str());
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
