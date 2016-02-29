#!/usr/bin/env gjs

const Gtk = imports.gi.Gtk;
const GLib = imports.gi.GLib;

Gtk.init(null, 0);

let bld = new Gtk.Builder();
bld.add_from_file("../global/ui.glade");

win = bld.get_object("appWindow");
win.show();

Gtk.main();
