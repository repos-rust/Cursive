extern crate cursive;

use cursive::Cursive;
use cursive::view::{IdView, TextView, Dialog, KeyEventView};

fn show_popup(siv: &mut Cursive) {

    siv.add_layer(Dialog::new(TextView::new("Tak!"))
                      .button("Change", |s| {
                          // Look for a view tagged "text". We _know_ it's there, so unwrap it.
                          let view = s.find_id::<TextView>("text").unwrap();
                          let content: String = view.get_content().chars().rev().collect();
                          view.set_content(&content);
                      })
                      .dismiss_button("Ok"));

}

fn main() {
    let mut siv = Cursive::new();

    let content = "Press Q to quit the application.\n\nPress P to open the popup.";

    siv.add_global_callback('q', |s| s.quit());

    // Let's wrap the view to give it a recognizable ID, so we can look for it.
    // We add the P callback on the textview only (and not globally),
    // so that we can't call it when the popup is already visible.
    siv.add_layer(KeyEventView::new(IdView::new("text", TextView::new(content)))
                      .register('p', |s| show_popup(s)));


    siv.run();
}
