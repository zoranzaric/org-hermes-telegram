use frankenstein::{api_params::ChatId, Api, GetUpdatesParams, SendMessageParams, TelegramApi};

fn main() {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(&token);

    let mut update_params = GetUpdatesParams::new();
    update_params.set_allowed_updates(Some(vec!["message".to_string()]));

    loop {
        let result = api.get_updates(&update_params);

        // println!("result: {:#?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let Some(message) = update.message() {
                        if let Some(text) = message.text.clone() {
                            println!("Text: {}", text);
                        } else {
                            println!("Unhandled message type");

                            let mut send_message_params = SendMessageParams::new(
                                ChatId::Integer(message.chat().id()),
                                "Thanks, I received your message!".to_string(),
                            );
                            send_message_params.set_reply_to_message_id(Some(message.message_id()));

                            let _ = api.send_message(&send_message_params);

                            update_params.set_offset(Some(update.update_id() + 1))
                        }
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}
