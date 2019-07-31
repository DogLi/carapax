use crate::{
    methods::Method,
    request::RequestBuilder,
    types::{ChatId, ChatPermissions, Integer},
};
use failure::Error;
use serde::Serialize;

/// Restrict a user in a supergroup
///
/// The bot must be an administrator in the supergroup
/// for this to work and must have the appropriate admin rights.
///
/// Pass True for all boolean parameters to lift restrictions from a user
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    chat_id: ChatId,
    user_id: Integer,
    permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<Integer>,
}

impl RestrictChatMember {
    /// Creates a new RestrictChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        RestrictChatMember {
            chat_id: chat_id.into(),
            user_id,
            permissions: ChatPermissions::default(),
            until_date: None,
        }
    }

    /// Replace current permissions with the new one
    pub fn with_permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }

    /// Restrict everything
    pub fn restrict_all(mut self) -> Self {
        self.permissions = ChatPermissions::restricted();
        self
    }

    /// Allow everything
    pub fn allow_all(mut self) -> Self {
        self.permissions = ChatPermissions::allowed();
        self
    }

    /// Date when restrictions will be lifted for the user, unix time
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever
    pub fn until_date(mut self, until_date: Integer) -> Self {
        self.until_date = Some(until_date);
        self
    }

    /// Pass True, if the user can send text messages, contacts, locations and venues
    ///
    /// Deprecated, to be removed in tgbot 0.5.0. Use with_permissions() instead.
    pub fn can_send_messages(mut self, can_send_messages: bool) -> Self {
        self.permissions = self.permissions.with_send_messages(can_send_messages);
        self
    }

    /// Pass True, if the user can send audios, documents, photos,
    /// videos, video notes and voice notes, implies can_send_messages
    ///
    /// Deprecated, to be removed in tgbot 0.5.0. Use with_permissions() instead.
    pub fn can_send_media_messages(mut self, can_send_media_messages: bool) -> Self {
        self.permissions = self.permissions.with_send_media_messages(can_send_media_messages);
        self
    }

    /// Pass True, if the user can send animations, games, stickers and
    /// use inline bots, implies can_send_media_messages
    ///
    /// Deprecated, to be removed in tgbot 0.5.0. Use with_permissions() instead.
    pub fn can_send_other_messages(mut self, can_send_other_messages: bool) -> Self {
        self.permissions = self.permissions.with_send_other_messages(can_send_other_messages);
        self
    }

    /// Pass True, if the user may add web page previews to their messages,
    /// implies can_send_media_messages
    ///
    /// Deprecated, to be removed in tgbot 0.5.0. Use with_permissions() instead.
    pub fn can_add_web_page_previews(mut self, can_add_web_page_previews: bool) -> Self {
        self.permissions = self.permissions.with_add_web_page_previews(can_add_web_page_previews);
        self
    }
}

impl Method for RestrictChatMember {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("restrictChatMember", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn restrict_chat_member_restrict_all() {
        let request = RestrictChatMember::new(1, 2)
            .restrict_all()
            .until_date(100)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/restrictChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": false,
                    "can_send_media_messages": false,
                    "can_send_polls": false,
                    "can_send_other_messages": false,
                    "can_add_web_page_previews": false,
                    "can_change_info": false,
                    "can_invite_users": false,
                    "can_pin_messages": false,
                })
            );
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }

    #[test]
    fn restrict_chat_member_allow_all() {
        let request = RestrictChatMember::new(1, 2)
            .allow_all()
            .until_date(100)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/restrictChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": true,
                    "can_send_media_messages": true,
                    "can_send_polls": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": true,
                    "can_change_info": true,
                    "can_invite_users": true,
                    "can_pin_messages": true,
                })
            );
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }

    #[test]
    fn restrict_chat_member_custom() {
        let request = RestrictChatMember::new(1, 2)
            .can_send_messages(true)
            .can_send_media_messages(false)
            .can_send_other_messages(true)
            .can_add_web_page_previews(false)
            .until_date(100)
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/restrictChatMember");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["user_id"], 2);
            assert_eq!(data["until_date"], 100);
            assert_eq!(
                data["permissions"],
                serde_json::json!({
                    "can_send_messages": true,
                    "can_send_media_messages": false,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": false
                })
            );
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
