use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String, // Add the 'name' field to the Paragraph struct
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work json in Rust"),
        author: String::from("om"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentence"),
            },
            Paragraph {
                name: String::from("body of the paragraph"),
            },
            Paragraph {
                name: String::from("end of the paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is:{}", json);
}
