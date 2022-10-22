use crate::app::components::inbox::ui::primary::show_inbox;
use cursive::views::Dialog;
use cursive::Cursive;

pub fn show_sent(s: &mut Cursive, with_message: bool) {
    s.set_autorefresh(false);
    s.pop_layer();
    let content;
    if with_message {
        content = "Message sent successfully!";
    } else {
        content = "Sent successfully!";
    }
    s.add_layer(Dialog::text(content).button("Back", |s| show_inbox(s)));
}
