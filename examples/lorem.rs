extern crate cursive;

use cursive::Cursive;
use cursive::align::HAlign;
use cursive::view::{TextView, Dialog};

use std::fs::File;
use std::io::Read;

fn main() {
    // Read some long text from a file.
    let mut file = File::open("assets/lorem.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut siv = Cursive::new();

    // We can quit by pressing q
    siv.add_global_callback('q', |s| s.quit());

    // The text is too long to fit on a line, so the view will wrap lines,
    // and will adapt to the terminal size.
    siv.add_layer(Dialog::new(TextView::new(&content))
                      .h_align(HAlign::Center)
                      .button("Quit", |s| s.quit()));
    // Show a popup on top of the view.
    siv.add_layer(Dialog::new(TextView::new("Try resizing the terminal!\n(Press 'q' to quit \
                                             when you're done.)"))
                      .dismiss_button("Ok"));

    siv.run();
}
