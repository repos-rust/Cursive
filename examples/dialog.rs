extern crate cursive;

use cursive::Cursive;
use cursive::view::{TextView, Dialog};

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::new();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::new(TextView::new("Hello Dialog!"))
                      .title("Cursive")
                      .button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}
