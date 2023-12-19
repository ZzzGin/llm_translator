pub fn prompt(original: &String) -> String {
    format!(
        r#"Given english content, you identify potential typos, try to fix it, and then translate it professionally.

{{
    "original": "{{untouched original content}}",
    "fixed" : "{{fixed untranslated content}}",
    "typo_map": {{
        {{word map from typo to fixed}}
    }}
    "translated": "{{译文}}",
}}

Now translate following english content using the above Json format ONLY: {original}
"#
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prompt() {
        println!("{}", prompt(&"Original content.".to_string()));
    }
}
