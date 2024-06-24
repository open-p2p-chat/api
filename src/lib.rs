pub mod api {
    use chat_p2p::{ChatP2P, Message};

    pub struct ChatAPI {
        chat_session: ChatP2P,
    }

    impl ChatAPI {
        pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let chat_session = ChatP2P::new().await?;
            Ok(Self { chat_session })
        }

        pub async fn create_offer(&self) -> Result<String, Box<dyn std::error::Error>> {
            self.chat_session.create_offer().await
        }

        pub async fn set_remote_description(&self, sdp: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.chat_session.set_remote_description(sdp).await
        }

        pub fn send_message(&self, message: Message) {
            self.chat_session.send_message(&message);
        }
    }
}
