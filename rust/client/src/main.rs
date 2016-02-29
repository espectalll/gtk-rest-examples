extern crate gtk;

use gtk::widgets::Builder;
use gtk::Window;
use gtk::signal::{Inhibit, WidgetSignals};
use gtk::traits::widget::WidgetTrait;

fn main() {
    gtk::init();
    let builder = Builder::new_from_file("../global/ui.glade").unwrap();
    
    unsafe {
        let window: Window = builder.get_object("appWindow").unwrap();
        
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        
        window.show_all();
    }

    gtk::main();
}
