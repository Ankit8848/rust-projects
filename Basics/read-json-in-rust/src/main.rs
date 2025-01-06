use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    title: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
        "article": "How to work with json in rust",
        "author": "ankit",
        "paragraph": [
            {
                "title": "hey there,"
            },
            {
                "title": "it's me."
            },
            {
                "title": "You're not alone."
            }
        ]
    }
    "#;

    let parsed = read_json_typed(json);

    println!(
        "\n\nThe title of the second paragraph is: {}",
        parsed.paragraph[1].title
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    serde_json::from_str(raw_json).expect("Failed to parse JSON")
}
