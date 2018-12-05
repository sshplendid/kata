#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // 반환 값은 self와 같은 lifetime을 가진다.
        3
    }

    fn announce_and_return_part(&self, announcemenet: &str) -> &str {
        println!("Attention please: {}", announcemenet);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };

    println!("first line is '{}.'", i.part);
    
    let part = i.announce_and_return_part("에헴");
    println!("part -> {}", part);
}
