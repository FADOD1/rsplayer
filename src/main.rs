mod ui;
mod config;
mod player;
mod functions;

use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;

fn main() {
    loop {
        let choices = vec![
            "Selecionar vídeo e reproduzir",
            "Mudar pasta padrão de vídeos",
            "Configurações",
            "Sair"
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("RSPlayer 📽️ - Escolha uma opção:")
            .default(0)
            .items(&choices)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let video_folder = config::load().video_folder;
                println!("Pasta de vídeos carregada: {:?}", video_folder); // debug

                let video_path = ui::select_video(&video_folder);
                println!("Resultado da seleção: {:?}", video_path); // debug

                if let Some(path) = video_path {
                    println!("Executando o player com o vídeo: {:?}", path); // debug
                    player::play_video(&path);
                } else {
                    println!("Nenhum vídeo foi selecionado!");
                }
            }
            1 => {
                let new_folder = ui::select_folder();
                if let Some(folder) = new_folder {
                    config::update_video_folder(folder);
                    println!("Pasta padrão atualizada com sucesso!");
                } else {
                    println!("Nenhuma pasta selecionada!");
                }
            }
            2 => {
                ui::config_menu();
            }
            3 => {
                println!("Até logo 🖖👾!");
                break;
            }
            _ => unreachable!(),
        }
    }
}