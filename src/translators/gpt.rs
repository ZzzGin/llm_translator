use std::fs;

use crate::{
    apis::gpt_api,
    models::{gpt::Message, translator_output::TranslatorOutput},
};

pub async fn translates(
    original_text: &String,
    prompt_path: &str,
) -> Result<TranslatorOutput, Box<dyn std::error::Error + Send>> {
    let prompt: String = fs::read_to_string(prompt_path).map_err(
        |e: std::io::Error| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        },
    )?;

    let message: Message = Message {
        role: "user".to_string(),
        content: prompt + original_text,
    };

    let gpt_response: String = gpt_api::call(vec![message]).await?;

    Ok(serde_json::from_str(gpt_response.as_str()).map_err(
        |e: serde_json::Error| -> Box<dyn std::error::Error + Send> {
            Box::new(e)
        },
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TEXT: &str = "Herman Melville (born Melvill; August 1, 1819 – September 28, 1891) was an Amrican noelist, short story writer, and poet of the American Renaissance period.";
    const TEST_PROMPT_PATH: &str = "./prompts/eng_to_chn.txt";

    #[tokio::test]
    async fn tests_ai_task_request() {
        let res: TranslatorOutput =
            translates(&TEST_TEXT.to_string(), TEST_PROMPT_PATH)
                .await
                .unwrap();

        dbg!(res);
    }
}
