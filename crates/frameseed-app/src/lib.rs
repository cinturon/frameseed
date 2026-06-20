use frameseed_core::{
    effects_from_config, frame_to_base64, load_preset, scene_from_config, Frame, RenderContext,
    Rgba,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PreviewRequest {
    preset: String,
    seed: u64,
    frame_index: Option<u32>,
}

#[tauri::command]
fn welcome() -> String {
    frameseed_core::welcome_message().to_string()
}

#[derive(Serialize)]
struct GalleryItem {
    slug: String,
    title: String,
}

#[tauri::command]
fn list_gallery() -> Vec<GalleryItem> {
    frameseed_core::gallery_entries()
        .iter()
        .map(|entry| GalleryItem {
            slug: entry.slug.to_string(),
            title: entry.title.to_string(),
        })
        .collect()
}

#[tauri::command]
fn preview_frame(request: PreviewRequest) -> Result<String, String> {
    let mut config = load_preset(&request.preset).map_err(|e| e.to_string())?;
    config.seed = request.seed;

    let frame_index = request.frame_index.unwrap_or(0);
    let total_frames = config.total_frames();

    let scene = scene_from_config(&config.scene).map_err(|e| e.to_string())?;
    let mut effects = effects_from_config(&config.effects);

    let mut frame = Frame::new(config.width, config.height);
    frame.clear(Rgba::black());
    
    let context = RenderContext::new(frame_index, total_frames, config.fps, config.seed);

    scene.render(&mut frame, &context);
    for effect in &mut effects {
        effect.apply(&mut frame, &context);
    }

    let base64 = frame_to_base64(&frame).map_err(|e| e.to_string())?;
    Ok(base64)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![welcome, list_gallery, preview_frame])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gallery_lists_twelve_presets_with_slugs() {
        let items = list_gallery();
        assert_eq!(items.len(), 12);
        assert_eq!(items[0].slug, "gradient");
        assert_eq!(items[0].title, "Gradient");
    }

    #[test]
    fn preview_frame_changes_with_seed_and_preset() {
        let noise_42 = preview_frame(PreviewRequest {
            preset: "noise_clouds".into(),
            seed: 42,
            frame_index: None,
        })
        .expect("noise preview");
        let noise_99 = preview_frame(PreviewRequest {
            preset: "noise_clouds".into(),
            seed: 99,
            frame_index: None,
        })
        .expect("noise preview other seed");
        let gradient_42 = preview_frame(PreviewRequest {
            preset: "gradient".into(),
            seed: 42,
            frame_index: None,
        })
        .expect("gradient preview");

        assert!(!noise_42.is_empty());
        assert_ne!(noise_42, noise_99, "seed change should change preview");
        assert_ne!(gradient_42, noise_42, "preset change should change preview");
    }
}
