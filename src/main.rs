use std::{
    error, io,
    process::{Command, Output},
};
use teloxide::{dispatching::dialogue::InMemStorage, net::Download, prelude::*};
use tokio::fs;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
const ARGS: &'static str = "--compute_type int8 --diarize --highlight_words True";

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

fn transcode(path: &str) -> &str {
    let output = Command::new("whisper").args([path, ARGS]).output();

    match output {
        Ok(output) => {
            // Convert the captured stdout to a String
            if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
                // Print each line from the command's output
                for line in stdout.lines() {
                    println!("{}", line);
                }
            } else {
                eprintln!("Failed to parse command output");
            }
        }
        Err(e) => {
            // Handle any errors that might occur while executing the command
            eprintln!("Command execution failed with error: {}", e);
        }
    }

    "test"
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    match msg.voice() {
        Some(p) => {
            let file_id = &p.file.id;
            let file = bot.get_file(file_id).await?;
            let path = format!("/tmp/{file_id}");
            let mut dst = fs::File::create(&path).await?;
            bot.download_file(&file.path, &mut dst).await?;
            let output = transcode(&path);
            bot.send_message(msg.chat.id, output).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send a voice memo")
                .await?;
        }
    }

    Ok(())
}
