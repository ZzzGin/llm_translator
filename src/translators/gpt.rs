use crate::{
    apis::gpt_api,
    models::{gpt::Message, translator_output::TranslatorOutput},
};

pub async fn translates(
    original_text: &String,
    prompt_text: fn(&String) -> String,
) -> Result<TranslatorOutput, Box<dyn std::error::Error + Send>> {
    let message: Message = Message {
        role: "user".to_string(),
        content: prompt_text(original_text),
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
    use crate::prompts::eng_to_chn;

    use super::*;

    #[tokio::test]
    async fn tests_ai_task_request() {
        let res: TranslatorOutput =
            translates(&"Have a good day".to_string(), eng_to_chn::prompt)
                .await
                .unwrap();

        dbg!(res);
    }
}
