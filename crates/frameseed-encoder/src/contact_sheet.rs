use font8x8::BASIC_FONTS;
use font8x8::UnicodeFonts;
use image::Rgba;
use image::RgbaImage;
use image::imageops;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

fn sampled_frame_indices(total_frames: u32, step: u32) -> Vec<u32> {
    if total_frames == 0 {
        return vec![];
    }
    let step = step.max(1);
    let mut indices: Vec<u32> = (1..=total_frames).step_by(step as usize).collect();

    if indices.last() != Some(&total_frames) {
        indices.push(total_frames);
    }
    indices
}

fn format_label(frame_number: u32, fps: f32, use_time: bool) -> String {
    if use_time {
        let time_seconds = (frame_number.saturating_sub(1) as f32) / fps;
        format!("{:.2}s", time_seconds)
    } else {
        format!("f:{}", frame_number)
    }
}

fn frame_file_path(sequence_dir: &Path, frame_number: u32) -> PathBuf {
    sequence_dir.join(format!("frame_{frame_number:06}.png"))
}

fn draw_label(sheet: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, text: &str) {
    for py in y..y + h {
        for px in x..x + w {
            sheet.put_pixel(px, py, Rgba([48, 48, 48, 255]));
        }
    }

    let text_color = Rgba([220, 220, 220, 255]);
    draw_text(sheet, x + 4, y + 4, text, text_color);
}

fn draw_text(sheet: &mut RgbaImage, x: u32, y: u32, text: &str, color: Rgba<u8>) {
    for (i, ch) in text.chars().enumerate() {
        draw_char(sheet, x + (i as u32 * 9), y, ch, color);
    }
}

fn draw_char(sheet: &mut RgbaImage, x: u32, y: u32, ch: char, color: Rgba<u8>) {
    let Some(glyph) = BASIC_FONTS.get(ch) else {
        return; // unsupported character — skip
    };
    for (row, &bits) in glyph.iter().enumerate() {
        for col in 0..8 {
            if bits & (1 << col) != 0 {
                sheet.put_pixel(x + col, y + row as u32, color);
            }
        }
    }
}

pub fn create_contact_sheet(
    sequence_dir: &Path,
    output_file: &Path,
    total_frames: u32,
    fps: f32,
    step: u32,
    cols: u32,
    thumb_width: u32,
) -> Result<(), Box<dyn Error>> {
    let indices = sampled_frame_indices(total_frames, step);

    let first_path = frame_file_path(sequence_dir, indices[0]);
    let first_image = image::open(&first_path)?;

    let (src_width, src_height) = (first_image.width(), first_image.height());

    let thumb_height = (thumb_width as f32 * src_height as f32 / src_width as f32).round() as u32;

    let thumb_count = indices.len() as u32;
    let cols = cols.max(1);
    let rows = thumb_count.div_ceil(cols);

    const LABEL_HEIGHT: u32 = 16;

    let cell_width = thumb_width;
    let cell_height = thumb_height + LABEL_HEIGHT;

    let sheet_width = cell_width * cols;
    let sheet_height = cell_height * rows;

    let mut sheet =
        image::RgbaImage::from_pixel(sheet_width, sheet_height, Rgba([32, 32, 32, 255]));

    for (i, &frame_number) in indices.iter().enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;

        let x = col * cell_width;
        let y = row * cell_height;

        let rgba = image::open(frame_file_path(sequence_dir, frame_number))?.to_rgba8();
        let scaled = image::imageops::resize(
            &rgba,
            thumb_width,
            thumb_height,
            image::imageops::FilterType::Triangle,
        );
        imageops::overlay(&mut sheet, &scaled, x as i64, y as i64);

        let label = format_label(frame_number, fps, false);
        draw_label(
            &mut sheet,
            x,
            y + thumb_height,
            cell_width,
            LABEL_HEIGHT,
            &label,
        );
    }

    sheet.save(output_file)?;
    Ok(())
}
