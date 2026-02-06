use tauri::image::Image;

/// Carrega o ícone do tray a partir do arquivo embutido
pub fn load_icon() -> Image<'static> {
    let icon_bytes = include_bytes!("../../icons/tray_white.png");
    let img = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (w, h) = img.dimensions();
    Image::new_owned(img.into_raw(), w, h)
}
