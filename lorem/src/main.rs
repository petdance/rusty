// https://github.com/gyscos/cursive/blob/master/examples/src/bin/lorem.rs

use cursive::align::HAlign;
use cursive::view::Scrollable;
use cursive::views::{Dialog, Panel, TextView};

fn main() {
    // Read some long text from a file.
    let content = include_str!("/Users/alester/hours.txt");

    let mut siv = cursive::default();

    // We can quit by pressing q
    siv.add_global_callback('q', |s| s.quit());

    // The text is too long to fit on a line, so the view will wrap lines,
    // and will adapt to the terminal size.
    siv.add_fullscreen_layer(
        Dialog::around(Panel::new(TextView::new(content).scrollable()))
            .title("Unicode and wide-character support")
            // This is the alignment for the button
            .h_align(HAlign::Center)
            .button("Quit", |s| s.quit()),
    );
    // Show a popup on top of the view.
    siv.add_layer(Dialog::info(
        "Try resizing the terminal!\n(Press 'q' to quit when you're done.)",
    ));

    siv.run();
}
