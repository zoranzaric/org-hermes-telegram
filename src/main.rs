use frankenstein::{
    api_params::ChatId, Api, GetUpdatesParams, Message, SendMessageParams, TelegramApi, Update,
};

fn main() {
    let api_url = std::env::var("ORG_HERMES_API_URL").expect("ORG_HERMES_API_URL not set");
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

                            match ureq::post(&api_url).send_json(ureq::json!({
                                "content": &text,
                            })) {
                                Ok(_) => accept_message(
                                    &mut update_params,
                                    &api,
                                    &message,
                                    update,
                                    "Thanks for your message!  I took note of it.".into(),
                                ),
                                Err(e) => {
                                    eprintln!("org-hermes-telegram: Error while sending a capture to the api: {}", e);
                                    accept_message(
                                        &mut update_params,
                                        &api,
                                        &message,
                                        update,
                                        "Thanks for your message!  Unfortunatly I could not take note of it.  I won't try again, please make sure to try again later.".into(),
                                    )
                                }
                            };
                        } else {
                            println!("Unhandled message type");
                            accept_message(&mut update_params, &api, &message, update, "Thanks for your message! Unfortunatly I am not able to handle the provide message type".into());
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

fn accept_message(
    update_params: &mut GetUpdatesParams,
    api: &Api,
    message: &Message,
    update: Update,
    response_text: String,
) {
    let mut send_message_params =
        SendMessageParams::new(ChatId::Integer(message.chat().id()), response_text);
    send_message_params.set_reply_to_message_id(Some(message.message_id()));

    update_params.set_offset(Some(update.update_id() + 1));

    let _ = api.send_message(&send_message_params);
}
