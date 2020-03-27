use std::fmt::Display;
use std::time::Duration;
use tokio::prelude::*;
use tokio::task::JoinHandle;
use tokio::time::delay_for;

async fn raw_print(input: impl Display) -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(&format!("{}", input).as_bytes()).await?;
    stdout.flush().await
}

enum Tick {
    Tick,
    NoTick,
}

async fn print_inner(input: &str, do_tick: Tick) -> io::Result<()> {
    for c in input.chars() {
        raw_print(c).await?;
        delay_for(Duration::from_millis(50)).await;
    }
    match do_tick {
        Tick::Tick => tick().await,
        Tick::NoTick => Ok(()),
    }
}

async fn print(input: &str) -> io::Result<()> {
    print_inner(input, Tick::Tick).await
}

async fn print_no_tick(input: &str) -> io::Result<()> {
    print_inner(input, Tick::NoTick).await
}

async fn tick() -> io::Result<()> {
    for _ in 0..4 {
        raw_print('.').await?;
        delay_for(Duration::from_millis(500)).await;
    }
    raw_print('\n').await
}

async fn read_characters() -> io::Result<String> {
    let mut output = String::new();
    io::BufReader::new(io::stdin()).read_line(&mut output).await?;
    Ok(output)
}

enum State {
    Input,
    Quitting,
    NotQuitting,
    Invalid,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    print("Hello Professor").await?;
    print("If you've made it this far, this is working").await?;
    print("I hope you are doing well during this realignment period").await?;
    print("It's been quite chaotic on my end, but this break should help").await?;
    print("Okay now I'm just showing off for the hell of it").await?;
    print(
        "Now as one final test, I'm going to get the source code of your website, asynchronously",
    )
    .await?;
    print("Again just for the hell of it :)").await?;

    let get_fut = tokio::spawn(reqwest::get("http://www.cs.hunter.cuny.edu/~eschweit/"));
    print("Accessing http://www.cs.hunter.cuny.edu/~eschweit/").await?;
    let response = get_fut.await.unwrap().unwrap();
    let text_fut: JoinHandle<reqwest::Result<String>> = tokio::spawn(response.text());
    print("Getting body").await?;
    let text = text_fut.await.unwrap().unwrap();
    print("Ah here it is").await?;
    raw_print(text).await?;

    print("Alright, we've had our fun").await?;

    let mut state = State::Input;
    loop {
        match state {
            State::Input => {
                print_no_tick("Goodbye? [y/n] ").await?;
                let input = read_characters().await?.to_lowercase();
                if input.contains('y') {
                    state = State::Quitting;
                } else if input.contains('n') {
                    state = State::NotQuitting;
                } else {
                    state = State::Invalid;
                }
            }
            State::Quitting => {
                print("Bye Bye").await?;
                break Ok(());
            }
            State::NotQuitting => {
                print("While I would love you to stay, I must go").await?;
                state = State::Input;
            }
            State::Invalid => {
                print("That's not 'y' or 'n'").await?;
                state = State::Input;
            }
        }
    }
}
