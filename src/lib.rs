pub mod api {
    use p2p::chat_p2p::ChatP2P;
    use chat::message::Message;
    use std::error::Error;

    pub struct ChatAPI {
        chat_session: ChatP2P,
    }

    impl ChatAPI {
        pub async fn new() -> Result<ChatAPI, Box<dyn Error>> {
            let chat_session = ChatP2P::new().await?;
            Ok(ChatAPI { chat_session })
        }

        pub async fn create_offer(&self) -> Result<String, Box<dyn Error>> {
            let offer = self.chat_session.create_offer().await?;
            Ok(offer)
        }

        pub async fn set_remote_description(&self, sdp: &str) -> Result<(), Box<dyn Error>> {
            self.chat_session.set_remote_description(sdp).await?;
            Ok(())
        }

        pub fn send_message(&self, message: Message) {
            self.chat_session.send_message(&message);
        }
    }
}
