use gtk4::gdk;
use gstreamer::prelude::*;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button, HeaderBar, Orientation, Picture, Label};

pub fn play_video(path: &std::path::PathBuf) {
    gstreamer::init().expect("Falha ao inicializar o GStreamer!");

    let app = Application::builder()
        .application_id("com.rsplayer.video")
        .build();

    let uri = format!("file://{}", path.display());

    app.connect_activate(move |app| {
        let playbin = gstreamer::ElementFactory::make("playbin")
            .property("uri", uri.clone())
            .build()
            .expect("Erro ao criar playbin");

        let video_sink = gstreamer::ElementFactory::make("gtk4paintablesink")
            .build()
            .expect("Erro ao criar gtk4paintablesink");

        playbin.set_property("video-sink", &video_sink);

        let paintable = video_sink
            .property::<gdk::Paintable>("paintable");

        let picture = Picture::new();
        picture.set_paintable(Some(paintable.as_ref()));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("RSPlayer - Reprodução de Vídeo")
            .default_width(800)
            .default_height(450)
            .build();

        let btn_play = Button::builder().label("▶️ Play").build();
        let btn_pause = Button::builder().label("⏸️ Pause").build();
        let btn_stop = Button::builder().label("⏹️ Stop").build();

        let pipeline_clone = playbin.clone();
        btn_play.connect_clicked(move |_| {
            pipeline_clone.set_state(gstreamer::State::Playing).unwrap();
        });

        let pipeline_clone = playbin.clone();
        btn_pause.connect_clicked(move |_| {
            pipeline_clone.set_state(gstreamer::State::Paused).unwrap();
        });

        let pipeline_clone = playbin.clone();
        btn_stop.connect_clicked(move |_| {
            pipeline_clone.set_state(gstreamer::State::Ready).unwrap();
        });

        let controls = gtk4::Box::new(Orientation::Horizontal, 0);
        controls.append(&btn_play);
        controls.append(&btn_pause);
        controls.append(&btn_stop);

        let main_layout = gtk4::Box::new(Orientation::Vertical, 0);
        main_layout.append(&picture);
        main_layout.append(&controls);

        // ✅ Código Corrigido Aqui ✅
        let header = HeaderBar::builder().build();
        let title_label = Label::new(Some("RSPlayer 📽️"));
        header.set_title_widget(Some(&title_label));

        window.set_titlebar(Some(&header));
        window.set_child(Some(&main_layout));
        window.show();

        playbin.set_state(gstreamer::State::Playing).unwrap();

        let pipeline_clone = playbin.clone();
        let app_clone = app.clone();
        window.connect_close_request(move |_| {
            pipeline_clone.set_state(gstreamer::State::Null).unwrap();
            app_clone.quit();
            glib::Propagation::Stop
        });
    });

    app.run();
}