extern crate gtk;
extern crate hyper;

use std::io::Read;

use gtk::widgets::Builder;
use gtk::{Button, Window, TextBuffer};
use gtk::signal::{Inhibit, WidgetSignals, ButtonSignals};
use gtk::traits::widget::WidgetTrait;
use gtk::traits::text_buffer::TextBufferTrait;

use hyper::Client;

fn main() {
    gtk::init();
    let builder = Builder::new_from_file("../../global/ui.glade").unwrap();
    
    unsafe {
        let window: Window = builder.get_object("appWindow").unwrap();
        let button: Button = builder.get_object("sendButton").unwrap();
        let input_buffer: TextBuffer =
            builder.get_object("inputBuffer").unwrap();
        let output_buffer: TextBuffer =
            builder.get_object("outputBuffer").unwrap();
        
        button.connect_clicked(move |_| {
            let client = Client::new();
            let mut res = client.post("http://localhost")
            	.body("Hey :3")
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
    }

    gtk::main();
}
