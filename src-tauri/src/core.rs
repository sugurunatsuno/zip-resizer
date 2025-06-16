use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use image::{DynamicImage, ImageOutputFormat};
use image::GenericImageView;
use rayon::prelude::*;
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct ResizeOptions {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub quality: u8,
}

impl Default for ResizeOptions {
    fn default() -> Self {
        Self { max_width: None, max_height: None, quality: 80 }
    }
}

impl ResizeOptions {
    pub fn new(
        max_width: Option<u32>,
        max_height: Option<u32>,
        quality: u8,
    ) -> Result<Self> {
        if quality > 100 {
            return Err(format!("quality must be between 0 and 100: {quality}").into());
        }
        Ok(Self { max_width, max_height, quality })
    }
}

fn is_image(name: &str) -> bool {
    matches!(Path::new(name).extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase()).as_deref(),
        Some("jpg") | Some("jpeg") | Some("png"))
}

fn resize_image(img: DynamicImage, opt: &ResizeOptions) -> DynamicImage {
    let (mut w, mut h) = img.dimensions();
    if let Some(max_w) = opt.max_width {
        if w > max_w {
            let ratio = max_w as f32 / w as f32;
            w = max_w;
            h = ((h as f32 * ratio) as u32).max(1);
        }
    }
    if let Some(max_h) = opt.max_height {
        if h > max_h {
            let ratio = max_h as f32 / h as f32;
            h = max_h;
            w = ((w as f32 * ratio) as u32).max(1);
        }
    }
    img.resize(w, h, image::imageops::FilterType::Lanczos3)
}

pub fn process_zip(input: &Path, output: &Path, opt: &ResizeOptions) -> Result<()> {
    let input_file = File::open(input)?;
    let mut archive = ZipArchive::new(input_file)?;

    let mut entries = Vec::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.is_dir() { continue; }
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        entries.push((file.name().to_string(), data));
    }

    let processed: Vec<(String, Vec<u8>)> = entries
        .into_par_iter()
        .map(|(name, data)| {
            if is_image(&name) {
                match image::load_from_memory(&data) {
                    Ok(img) => {
                        let img = resize_image(img, opt);
                        let mut buf = std::io::Cursor::new(Vec::new());
                        if let Err(e) = img.write_to(&mut buf, ImageOutputFormat::Jpeg(opt.quality)) {
                            eprintln!("Failed to encode {name}: {e}");
                            return (name, data);
                        }
                        (name, buf.into_inner())
                    }
                    Err(e) => {
                        eprintln!("Failed to decode {name}: {e}");
                        (name, data)
                    }
                }
            } else {
                (name, data)
            }
        })
        .collect();

    let output_file = File::create(output)?;
    let mut writer = ZipWriter::new(output_file);
    let options = FileOptions::default();

    for (idx, (name, data)) in processed.into_iter().enumerate() {
        println!("Processing {}", name);
        writer.start_file(name, options)?;
        writer.write_all(&data)?;
    }

    writer.finish()?;
    Ok(())
}
