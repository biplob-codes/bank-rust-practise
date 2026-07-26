#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}
fn print_media(m: Media) {
    println!("{:#?}", m)
}
impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} - {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} - {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format! {"{}",title}
        } else {
            String::from("Media description")
        }
    }
}
fn main() {
    let ab = Media::Audiobook {
        title: String::from("Sharing is caring"),
    };
    let b = Media::Book {
        title: String::from("Operating Systems: Three Easy Pieces"),
        author: String::from("Remzi and Andrea Arpaci-Dusseau"),
    };

    let m = Media::Movie {
        title: String::from("The Oddessy"),
        director: String::from("Christoger Nolan"),
    };

    println!("{}", ab.description());
    println!("{}", b.description());
    println!("{}", m.description());
}
