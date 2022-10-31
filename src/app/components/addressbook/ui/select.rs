use crate::app::{constants::AUTHOR, userdata::UserData};
use cursive::{
    views::{Dialog, HideableView, TextArea},
    Cursive,
};

pub fn select_addressbook(s: &mut Cursive, name: &str) {
    s.pop_layer();
    let data = &s.user_data::<UserData>().unwrap();
    let address = if name == AUTHOR {
        let network_prefix = (&data.coins[data.coin_idx].prefix).clone();
        network_prefix + "3kpznqbuzs3grswcqkzitd5fwky4s5cmyt76wru7kbenfwza7q9c1f1egzhm"
    } else {
        let address = data.addressbook.get(name).unwrap().clone();
        let idx_of_ = address.find('_').unwrap();
        let prefix = &address[..idx_of_ + 1];
        let network_prefix = (&data.coins[data.coin_idx].prefix).clone();
        if network_prefix != prefix {
            let non_prefix = &address[idx_of_ + 1..];
            s.add_layer(Dialog::info(format!("The contact's address originally began {} but was automatically updated to {} for the current network.", prefix, network_prefix)));
            network_prefix + non_prefix
        } else {
            address
        }
    };
    s.call_on_name("hideable", |view: &mut HideableView<Dialog>| {
        view.set_visible(true);
    })
    .unwrap();
    s.call_on_name("address", |view: &mut TextArea| {
        view.set_content(address);
    })
    .unwrap();
}
