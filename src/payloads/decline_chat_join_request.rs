// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the _can_invite_users_ administrator right. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub DeclineChatJoinRequest (DeclineChatJoinRequestSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier of the target user
            pub user_id: i64,
        }
    }
}