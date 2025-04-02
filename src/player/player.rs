use gstreamer::prelude::*;
use std::path::PathBuf;

pub fn play_video(path: &PathBuf) {
    if let Err(e) = gstreamer::init() {
        eprintln!("Erro ao iniciar o GStreamer: {:?}", e);
        return;
    }

    let uri = format!("file://{}", path.display());
    println!("URI gerada para reprodução: {}", uri); // debug

    let playbin = match gstreamer::ElementFactory::make("playbin")
        .property("uri", &uri)
        .build() {
        Ok(elem) => elem,
        Err(e) => {
            eprintln!("Erro criando playbin: {:?}", e);
            return;
        }
    };

    let bus = match playbin.bus() {
        Some(bus) => bus,
        None => {
            eprintln!("Não foi possível obter o bus do pipeline");
            return;
        }
    };

    playbin.set_state(gstreamer::State::Playing).expect("Não foi possível iniciar reprodução");

    // Loop básico de eventos para acompanhar execução
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            gstreamer::MessageView::Eos(..) => {
                println!("Fim da reprodução!");
                break;
            }
            gstreamer::MessageView::Error(err) => {
                eprintln!("Erro reproduzindo vídeo: {:?}", err.error());
                if let Some(debug) = err.debug() {
                    eprintln!("Informação extra: {}", debug);
                }
                break;
            }
            _ => (),
        }
    }

    playbin.set_state(gstreamer::State::Null).unwrap();
    println!("Player finalizado corretamente!");
}