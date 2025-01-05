use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article{
        title: String::from("How to work with json in rust"),
        author: String::from("ankit"),
        paragraph: vec![
            Paragraph {
                content: String::from("First sentence."),

            },
            Paragraph {
                content: String::from("Body of the paragraph."),
            },
            Paragraph {
                content: String::from("End of paragraph."),
            },
        ],
    };

    println!("{}", serde_json::to_string_pretty(&article).unwrap());
}
