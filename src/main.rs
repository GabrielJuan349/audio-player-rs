mod ui;
use gstreamer as gst;

use gst::prelude::*;
use gtk::prelude::*;
use gtk::Application;

fn main() {

    gst::init().unwrap();

    let app = Application::new(Some("com.example.audio_player"), Default::default())
        .expect("failed to initialize GTK application");
    app.connect_activate(|app| {
        ui::build_ui(app);
    });
    app.run(&[]);
}
