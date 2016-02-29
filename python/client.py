#!/usr/bin/env python

import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk

builder = Gtk.Builder()
builder.add_from_file("../global/ui.glade")

window = builder.get_object("appWindow")
window.show_all()

Gtk.main()
