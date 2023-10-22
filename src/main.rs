use regex::Regex;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct PhoneticEntry {
    text: Option<String>,
    audio: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct DictionaryEntry {
    word: String,
    phonetic: Option<String>,
    phonetics: Vec<PhoneticEntry>,
    // origin: String,
    meanings: Vec<Meaning>,
}

impl fmt::Display for Meaning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\x1b[;1m({}\x1b[0m)", self.part_of_speech)?;
        for (idx, def) in self.definitions.iter().enumerate() {
            writeln!(f, "{}. {}", idx + 1, def.definition)?;
            if let Some(xmpl) = def.example.clone() {
                writeln!(f, "   \x1b[;3m\"{xmpl}\"\x1b[0m")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for DictionaryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let main_word_col = "\x1b[;1m";
        let reset = "\x1b[0m";
        let phonetics_col = "\x1b[0m";
        let ansi_regex = Regex::new(r#"\x1b\[.*?m"#).unwrap();
        let mut final_string = String::new();
        final_string.push_str(&format!("{main_word_col}{}{reset}", self.word));
        if let Some(p) = &self.phonetic {
            final_string.push_str(&format!("{phonetics_col}[\"{p}\"]{reset}"));
        }
        writeln!(f, "{final_string}")?;

        // .chars().count() != len(), as the latter counts unicode modifyers as own
        // characters even if they display as one
        let separator_length = ansi_regex.replace_all(&final_string, "").chars().count();
        let separator = format!("\x1b[90m{}\x1b[0m", "-".repeat(separator_length));
        writeln!(f, "{separator}")?;

        for meaning in &self.meanings {
            write!(f, "{meaning}")?;
            writeln!(f, "{separator}")?;
        }
        write!(f, "")
    }
}

#[tokio::main]
async fn main() {
    let api_url = "https://api.dictionaryapi.dev/api/v2/entries/en";
    let word = std::env::args().nth(1).unwrap();
    let resp_obj = reqwest::get(format!("{api_url}/{word}")).await.unwrap();

    let entries = resp_obj.json::<Vec<DictionaryEntry>>().await.unwrap();

    for entry in &entries {
        println!("{entry}");
    }
}
