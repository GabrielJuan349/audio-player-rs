use gtk::prelude::*;
use gstreamer as gst;
use gstreamer::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog};
use gtk::glib::clone;

pub fn build_ui(app: &Application){
    let window = ApplicationWindow::new(app);
    window.set_title("Audio-Player-rs");
    window.set_default_size(350, 100);
    let play_button = Button::with_label("Play");
    let pause_button = Button::with_label("Pause");
    let stop_button = Button::with_label("Stop");
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&play_button, true, true, 0);
    vbox.pack_start(&pause_button, true, true, 0);
    vbox.pack_start(&stop_button, true, true, 0);
    window.add(&vbox);
    window.show_all();

    // Event Handler
    let pipeline = gst::Pipeline::new();
    let playbin = gst::ElementFactory::make("playbin").unwrap();
    pipeline.add(&playbin).unwrap();
    play_button.connect_clicked(clone!(@weak playbin => move |_| {
        let dialog = FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            FileChooserAction::Open,
            //&[("_Open", gtk::ResponseType::Accept), ("_Cancel", gtk::ResponseType::Cancel)],
        );
        if dialog.run() == gtk::ResponseType::Accept.into() {
            if let Some(file) = dialog.get_filename() {
                let uri = format!("file://{}", file.to_str().unwrap());
                playbin.set_property("uri", &uri).unwrap();
                playbin.set_state(gst::State::Playing).unwrap();
            }
        }
        dialog.close();
    }));
    pause_button.connect_clicked(clone!(@weak playbin => move |_| {
        playbin.set_state(gst::State::Paused).unwrap();
    }));
    stop_button.connect_clicked(clone!(@weak playbin => move |_| {
        playbin.set_state(gst::State::Null).unwrap();
    }));
}