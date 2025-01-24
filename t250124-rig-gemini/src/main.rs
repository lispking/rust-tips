use rig::{completion::Prompt, providers};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = providers::gemini::Client::from_env();
    let agent = client
        .agent("gemini-1.5-pro")
        .preamble("You are a helpful assistant.")
        .build();

    let answer = agent.prompt("Tell me a joke").await?;
    println!("Answer: {}", answer);
    Ok(())
}
