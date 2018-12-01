pub trait Summarizable {
    fn summary(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String, 
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use Tweet;
    use Summarizable;
    #[test]
    fn tweet_test() {
        let tweet = Tweet {
            username: String::from("via_ssh"),
            content: String::from("I'm learning Rust Language."),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summary());
        
        let expected = format!("{}: {}", tweet.username, tweet.content);
        assert_eq!(tweet.summary(), expected);
    }
}
