use std::collections::HashMap;
use crate::Handler;

pub fn call(action: &str, handler: Handler, peer: &mut enet::Peer<()>, data: HashMap<String, String>) {
    match action {
        "refresh_item_data" => handler.on_request_items_dat(peer, None),
        "enter_game" => handler.on_enter_game(peer, None),
        "join_request" => handler.on_join_request(peer, Some(data)),
        _ => println!("Unhandled action: {}\nData: {:?}", action, data)
    };
}