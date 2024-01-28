use std::process::Command;
use teloxide::{dispatching::dialogue::InMemStorage, net::Download, prelude::*};
use tokio::fs;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting text to speech bot...");

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

async fn transcode(path: &str) -> String {
    let output = Command::new("whisper")
        .args([path, "--output_format", "txt", "--output_dir", "/tmp"])
        .current_dir("/tmp")
        .status();

    println!("a1");

    match output {
        Ok(_output) => match fs::read_to_string(format!("{path}.txt")).await {
            Ok(result) => {
                log::info!("Result for {path}: {result}");
                return result;
            }
            Err(_) => {
                log::error!("Counld not parse an audio by given path: {path}");
                return String::new();
            }
        },
        Err(_) => {
            log::error!("Counld not parse an audio by given path: {path}");
            return String::new();
        }
    }
}

async fn start(bot: Bot, msg: Message) -> HandlerResult {
    match msg.voice() {
        Some(p) => {
            let initial_response = bot
                .send_message(msg.chat.id, "Started processing 💪")
                .reply_to_message_id(msg.id)
                .await
                .unwrap();
            let file_id = &p.file.id;
            let file = bot.get_file(file_id).await?;
            let path = format!("/tmp/{file_id}");
            let mut dst = fs::File::create(&path).await?;
            bot.download_file(&file.path, &mut dst).await?;
            let mut output = transcode(&path).await;
            if output.is_empty() {
                output = "Could not parse a voice message".to_string();
            }
            let _ = bot
                .edit_message_text(msg.chat.id, initial_response.id, output)
                .await;
        }
        None => {
            bot.send_message(msg.chat.id, "Please send a voice memo")
                .await?;
        }
    }

    Ok(())
}
