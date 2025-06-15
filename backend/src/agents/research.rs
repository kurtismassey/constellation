use crate::agents::tools::search::WebSearch;
use crate::config::Settings;
use rig::{client::CompletionClient, completion::Prompt, providers::openai};

pub async fn research(query: String) -> String {
    let llm = openai::Client::new(&Settings::new().openai_api_key);

    let research_agent = llm
        .agent(openai::GPT_4)
        .preamble(include_str!("prompts/research.txt"))
        .tool(WebSearch)
        .build();

    let response = research_agent
        .prompt(query)
        .await
        .expect("Constellation research agent failed to respond");

    response.to_string()
}
