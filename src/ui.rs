use dialoguer::{theme::ColorfulTheme, Select, Input};
use std::{fs, path::PathBuf};
use std::println;

pub fn select_video(default_folder: &str) -> Option<PathBuf> {
    let folder_path = shellexpand::tilde(default_folder).to_string();
    let paths = fs::read_dir(folder_path).ok()?.filter_map(|f| {
        let path = f.ok()?.path();
        let ext = path.extension()?.to_string_lossy().to_lowercase();
        if ["mp4", "avi", "mkv", "mov"].contains(&ext.as_ref()) {
            Some(path)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    if paths.is_empty() {
        println!("Nenhum vídeo encontrado na pasta.");
        return None;
    }

    let items = paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>();
    let i = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Escolha o vídeo para reproduzir:")
        .default(0)
        .items(&items)
        .interact().ok()?;

    Some(paths[i].clone())
}

pub fn select_folder() -> Option<PathBuf> {
    let folder: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Informe o novo caminho para vídeos")
        .interact_text()
        .ok()?;

    let folder = PathBuf::from(shellexpand::tilde(&folder).to_string());

    if folder.is_dir() { Some(folder) } else { None }
}

pub fn config_menu() {
    println!("⚙️ Função ainda não implementada.");
}