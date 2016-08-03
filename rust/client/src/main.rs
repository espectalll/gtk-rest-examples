extern crate gtk;
extern crate hyper;

use std::io::Read;

use gtk::prelude::*;
use gtk::{Button, Window, TextBuffer, Builder};

use hyper::Client;

fn main() {
    gtk::init();

    let builder = Builder::new_from_file("../../global/ui.glade");
    let window: Window = builder.get_object("appWindow").unwrap();
    let button: Button = builder.get_object("sendButton").unwrap();
    let input_buffer: TextBuffer = builder.get_object("inputBuffer").unwrap();
    let output_buffer: TextBuffer = builder.get_object("outputBuffer").unwrap();
    
    button.connect_clicked(move |_| {
        let client = Client::new();
        let mut data = input_buffer.get_text(
                           &input_buffer.get_start_iter(),
                           &input_buffer.get_end_iter(),
                           false).unwrap();
        let mut res = client.post("http://localhost")
            .body(data.as_bytes())
            .send()
            .unwrap();
        assert_eq!(res.status, hyper::Ok);
        let mut output = String::new();
        res.read_to_string(&mut output);
        output_buffer.set_text(&output);
    });
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
