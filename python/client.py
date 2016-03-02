#!/usr/bin/env python

import gi, requests
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

server = "http://localhost"

builder = Gtk.Builder()
builder.add_from_file("../global/ui.glade")

window = builder.get_object("appWindow")
button = builder.get_object("sendButton")
in_buffer = builder.get_object("inputBuffer")
out_buffer = builder.get_object("outputBuffer")

def on_button_clicked(self):
    in_data = in_buffer.get_text(
        in_buffer.get_start_iter(),
        in_buffer.get_end_iter(), True);
    r = requests.post(server,
                      headers = {'Content-Type': 'text/plain'},
                      data = in_data)
    out_buffer.set_text(r.text, len(r.text))
    
button.connect("clicked", on_button_clicked)

window.show_all()
Gtk.main()
