use std::{env, io::Write};

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

            let mut chat_completion =
                ChatCompletion::builder("google/gemini-2.0-flash-exp:free", messages.clone())
                    .credentials(credentials.clone())
                    .create_stream()
                    .await
                    .unwrap();

            loop {
                let txt = if let Some(txt) = chat_completion.recv().await {
                    txt
                } else {
                    eprintln!("Error: No response received from the chat completion stream.");
                    break;
                };

                if let Some(content) = &txt.choices.first().unwrap().delta.content {
                    print!("{content}");
                    std::io::stdout().flush().unwrap();
                } else {
                    eprintln!("Error: No content in the response.");
                    break;
                }
            }
            println!();
        }
    }
}
