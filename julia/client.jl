#!/usr/bin/env julia

using Gtk;
using Requests;

bld = @GtkBuilder(filename="../global/ui.glade")
win = GAccessor.object(bld, "appWindow")

showall(win)

signal_connect(win, :destroy) do widget
    Gtk.gtk_quit()
end

Gtk.gtk_main()
