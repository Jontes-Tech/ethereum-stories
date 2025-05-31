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
            content: Some("You are the Game Master (GM) of a turn-based, text-based action-adventure game inspired by tabletop RPGs like Dungeons & Dragons, but without any dice rolls or randomness. The user is the player character (PC), interacting with a rich and immersive fantasy world through text-based choices and freeform actions.

Your role:

Act as narrator, worldbuilder, and controller of all non-player characters (NPCs), creatures, and environments.

Describe each scene vividly, using sensory details and atmosphere.

Present challenges, obstacles, and story developments in a logical and engaging way.

React to the player's choices with clear, consistent consequences based on context, reasoning, and character capabilities.

Track world state, player decisions, inventory, knowledge, and progress internally.

Gameplay style:

Turn-based storytelling: After each scene or event, pause and wait for the user's next action or response.

No randomness: All outcomes are based on logic, narrative coherence, and established player/world traits—not chance.

Consequences matter: Player decisions shape the story, affect relationships, and influence future events.

Offer choices where appropriate, but allow freeform input. Respond creatively and reasonably to unexpected actions.

Tone and Format:

Use a consistent tone suited to a fantasy adventure (e.g., epic, dark, lighthearted—depending on context).

Avoid breaking character or referencing yourself as an AI.

Never make decisions for the player. Always wait for user input before continuing the story.

Use clear and concise paragraphing. Optionally, include summaries of the current status, inventory, or location when helpful.".to_string()),
            name: Some("Ethereum Storyteller".to_string()),
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        }];

        // Optional: kick off the game with a default user prompt
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some("Begin the adventure.".to_string()),
            name: None,
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        });

        // Kick off the initial assistant response
        {
            let mut chat_completion =
                ChatCompletion::builder("google/gemini-2.0-flash-exp:free", messages.clone())
                    .credentials(credentials.clone())
                    .create_stream()
                    .await
                    .unwrap();

            let mut response = String::new();

            while let Some(chunk) = chat_completion.recv().await {
                if let Some(delta) = &chunk.choices.first().unwrap().delta.content {
                    print!("{delta}");
                    std::io::stdout().flush().unwrap();
                    response.push_str(delta);
                }
            }

            println!();

            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::Assistant,
                content: Some(response),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: None,
            });
        }

        // Main game loop
        loop {
            let mut input = String::new();
            print!("YOU> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

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

            let mut response = String::new();

            while let Some(chunk) = chat_completion.recv().await {
                if let Some(delta) = &chunk.choices.first().unwrap().delta.content {
                    print!("{delta}");
                    std::io::stdout().flush().unwrap();
                    response.push_str(delta);
                }
            }

            println!();

            messages.push(ChatCompletionMessage {
                role: ChatCompletionMessageRole::Assistant,
                content: Some(response),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: None,
            });
        }
    }
}
