#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}
fn print_media(m: Media) {
    println!("{:#?}", m)
}

fn main() {
    let ab = Media::Audiobook {
        title: String::from("Sharing is caring"),
    };
    print_media(ab);
}
