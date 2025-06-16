pub mod core;
pub use core::{process_zip, ResizeOptions};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuiOptions {
    max_width: Option<u32>,
    max_height: Option<u32>,
    quality: Option<u8>,
}

fn build_options(opt: Option<GuiOptions>) -> Result<ResizeOptions, String> {
    match opt {
        Some(o) => {
            let q = o.quality.unwrap_or(80);
            ResizeOptions::new(o.max_width, o.max_height, q).map_err(|e| e.to_string())
        }
        None => Ok(ResizeOptions::default()),
    }
}

// Sample Tauri command. Keep for GUI testing.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn process_zips(paths: Vec<String>, options: Option<GuiOptions>) -> Result<(), String> {
    let opts = build_options(options)?;
    tauri::async_runtime::spawn_blocking(move || {
        for p in paths {
            let input = std::path::PathBuf::from(&p);
            let out_name = input
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "output".into());
            let output = input.with_file_name(format!("{out_name}_resized.zip"));

            if let Err(e) = process_zip(&input, &output, &opts) {
                return Err(e.to_string());
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn process_zip_cmd(path: String, options: Option<GuiOptions>) -> Result<(), String> {
    let opts = build_options(options)?;
    tauri::async_runtime::spawn_blocking(move || {
        let input = std::path::PathBuf::from(&path);
        let out_name = input
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "output".into());
        let output = input.with_file_name(format!("{out_name}_resized.zip"));
        process_zip(&input, &output, &opts).map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            process_zips,
            process_zip_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
