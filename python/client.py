#!/usr/bin/env python

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

builder = Gtk.Builder()
builder.add_from_file("../global/ui.glade")

window = builder.get_object("appWindow")
button = builder.get_object("sendButton")
in_buffer = builder.get_object("inBuffer")
out_buffer = builder.get_object("outBuffer")

def on_button_clicked(self):
    print("Hey!")
    
button.connect("clicked", on_button_clicked)

window.show_all()
Gtk.main()
