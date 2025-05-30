use std::env;

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    Credentials,
};

use crate::Scene;

impl Scene {
    pub async fn run(&self) {
        let credentials = Credentials::new(
            env::var("OPENROUTER_KEY").expect("OPENROUTER_KEY must be defined"),
            "https://openrouter.ai/api/v1",
        );

        let mut messages = vec![ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("You are the system of a cave explorer game.".to_string()),
            name: Some("Ethereum Storyteller".to_string()),
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        }];

        loop {
            let mut input = String::new();
            println!("What do you want to do?");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();

            let user_message = ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(input.to_string()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: None,
            };

            messages.push(user_message);

            let chat_completion =
                ChatCompletion::builder("deepseek/deepseek-r1-0528:free", messages.clone())
                    .credentials(credentials.clone())
                    .create()
                    .await
                    .unwrap();

            let returned_message = chat_completion.choices.first().unwrap().message.clone();

            messages.push(returned_message.clone());

            if let Some(content) = returned_message.content {
                println!("Response: {}", content);
            } else {
                println!("No content in response");
            }
        }
    }
}
