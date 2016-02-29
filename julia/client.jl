#!/usr/bin/env julia

using Gtk
using Requests
import Requests: post

const server = "http://localhost"

bld = @GtkBuilder(filename="../global/ui.glade")
win = GAccessor.object(bld, "appWindow")
btn = GAccessor.object(bld, "sendButton")
ibuff = GAccessor.object(bld, "inputBuffer")
obuff = GAccessor.object(bld, "outputBuffer")

signal_connect(win, :destroy) do widget
    Gtk.gtk_quit()
end
signal_connect(btn, :clicked) do widget
    idata = getproperty(ibuff, :text, AbstractString)
    res = post(server,
            headers = Dict("Content-Type" => "text/plain"),
            data = idata)
    setproperty!(obuff, :text, readall(res))
end

showall(win)
Gtk.gtk_main()
