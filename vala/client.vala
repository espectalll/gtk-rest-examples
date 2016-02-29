using Gtk;
using Soup;

public void on_btn_clicked() {
    stdout.printf("Hey");
    var xhrs = new Soup.Session();
    
    var bld = new Gtk.Builder();
    bld.add_from_file("../global/ui.glade");
    
    var ibuff = bld.get_object("inputBuffer") as TextBuffer;
    var obuff = bld.get_object("outputBuffer") as TextBuffer;
    
    Gtk.TextIter ibuff_start, ibuff_end;
    ibuff.get_start_iter(out ibuff_start);
    ibuff.get_end_iter(out ibuff_end);
    
    var idata = ibuff.get_text(ibuff_start, ibuff_end, true);
}

int main(string[] args) {
    Gtk.init(ref args);
    
    var bld = new Gtk.Builder();
    bld.add_from_file("../global/ui.glade");
    
    var win = bld.get_object("appWindow") as Window;
    var btn = bld.get_object("sendButton") as Button;
    
    btn.clicked.connect(on_btn_clicked);
    
    win.show_all();
    Gtk.main();
    
    return 0;
}
