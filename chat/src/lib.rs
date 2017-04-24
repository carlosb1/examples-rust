mod model {
    pub struct User {
        pub name : String,
    }    
    pub struct Message {
        pub author: String,
        pub message: String,
    }
    pub struct Chat {
        pub name: String,
    }
}

#[test]
fn given_an_user_where_is_initialised_then_Ok() {
    let mut user = model::User{name: "test".to_string()};
    assert_eq!(user.name,"test");
}

#[test]
fn given_a_chat_where_is_initialised_then_Ok() {
    let mut chat = model::Chat{name: "testchat".to_string()};
    assert_eq!(chat.name,"testchat");
}

#[test]
fn given_a_message_where_is_initialised_then_Ok() {
    let mut message = model::Message{author: "authortest".to_string(), message: "message".to_string()};
    assert_eq!(message.author,"authortest");
    assert_eq!(message.message,"message");
}



