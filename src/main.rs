fn main() {
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Renamer",
        native_options,
        Box::new(|cc| Box::new(file_renamer::RenamerApp::new(cc))),
    );
}
