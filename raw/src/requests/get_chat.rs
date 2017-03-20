use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChat<'c> {
    pub chat_id: ChatId<'c>
}

impl<'c> Request for GetChat<'c> {
    type Response = Chat;

    fn name(&self) -> &'static str {
        "getChat"
    }
}

impl<'c> GetChat<'c> {
    pub fn new<C>(chat: C) -> Self where C: Into<ChatId<'c>> {
        GetChat {
            chat_id: chat.into()
        }
    }
}

pub trait CanGetChat<'bc, 'c> {
    fn get_chat(&'bc self) -> GetChat<'c>;
}

impl<'c, 'bc, C: 'bc> CanGetChat<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn get_chat(&'bc self) -> GetChat<'c> {
        GetChat::new(self)
    }
}
