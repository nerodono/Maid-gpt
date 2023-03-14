pub fn updates_to_string(u: &[AllowedUpdate]) -> String {
    let mut s = String::from("[");

    for &update in u {
        let enum_str = match update {
            AllowedUpdate::CallbackQuery => "\"callback_query\"",
            AllowedUpdate::EditedChannelPost => {
                "\"edited_channel_post\""
            }
            AllowedUpdate::Message => "\"message\"",
        };

        s.push_str(enum_str);
        s.push(',');
    }

    s.pop().unwrap();
    s.push(']');
    s
}

mod details {
    use super::updates_to_string;
    use crate::{
        error::*,
        schemas::*,
    };

    include!(concat!(env!("OUT_DIR"), "/builders_generated.rs"));
}

pub use details::*;

use crate::schemas::AllowedUpdate;
