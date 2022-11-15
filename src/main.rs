use std::fs::File;
use std::io::Read;
use reqwest;
use serde_json::json;

pub struct Flashcard {
    question: String,
    answer: String,
}

impl Flashcard {
    pub fn new(question: String, answer: String) -> Flashcard {
        Flashcard {
            question: question,
            answer: answer,
        }
    }
}
#[tokio::main]
async fn main() {
    let flashcards: Vec<Flashcard> = parse_markdown(r#"---"#.to_string(), "src/flashcards.md".to_string());
    for flashcard in flashcards  {
        let api_url: String = "https://app.mochi.cards/api/cards".to_string();
        let username =  "" // API_KEY for PRO Users of mochi
        let deck_id = "".to_string();
        let formatted_content = format_flashcards(flashcard);
        let res = send_post_request(api_url, username, deck_id, formatted_content).await;
        println!("{:?}", res);
    }
}

// a function that imports a markdown file and returns a string
fn import_markdown(markdown_path: String) -> String {
    let mut file = File::open(markdown_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    contents
}
// if not example

// a function that parses the markdown file and returns a vector of flashcards
fn parse_markdown(delimiter: String, markdown_path: String) -> Vec<Flashcard> {
    let mut flashcards = Vec::new();
    let contents = import_markdown(markdown_path);
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("#") {
            let question = line.trim_start_matches("#");
            lines.next().unwrap();
            if !line.starts_with(&delimiter) {
                let answer = lines.next().unwrap();
                flashcards.push(Flashcard::new(question.to_string(), answer.to_string()));
            }
        }
    }
    flashcards
}


// a function to format output from parse_markdown function into a string
fn format_flashcards(flashcard: Flashcard) -> String {
    let mut flashcards_string = String::new();
    flashcards_string.push_str(&format!("# {} \n", flashcard.question));
    flashcards_string.push_str("---\n");
    flashcards_string.push_str(&format!("{}", flashcard.answer));
    flashcards_string
}

// a function to send POST request to https://app.mochi.cards/api/ using HTTP Basic Auth, just the username, 
// parameters should be: username, deck_id, formatted_content string
async fn send_post_request(api_url: String, username: String, deck_id: String, formatted_content: String) {
    let client = reqwest::Client::new();
    let json = json!({
        "deck-id": deck_id,
        "content": formatted_content,
    });
    let response = client.post(&api_url)
        .basic_auth(username, Some(""))
        .json(&json)
        .send()
        .await.unwrap();
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    let body: String = response.text().await.unwrap();
    println!("Body:\n{}", body);
}