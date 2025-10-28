mod games;
mod storage;
mod ui;

use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let storage_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".memorykata")
        .join("save.json");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MemoryKata",
        native_options,
        Box::new(move |_cc| {
            let storage = storage::Storage::new(storage_path.clone());
            Box::new(ui::App::new(storage))
        }),
    )?;

    Ok(())
}
