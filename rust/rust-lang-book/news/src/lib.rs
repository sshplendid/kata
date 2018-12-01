pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
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

pub struct Blog {
    pub author: String,
    pub title: String,
    pub content: String,
    pub link: String,
}

impl Summarizable for Blog {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use Summarizable;
    use Tweet;
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

    use NewsArticle;
    #[test]
    fn news_test() {
        let article = NewsArticle {
            headline: String::from("Rust in AWS!"),
            location: String::from("ZD Net"),
            author: String::from("Jeff"),
            content: String::from("Rust in AWS now!"),
        };

        let expected = format!("{}, by {} ({})", article.headline, article.author, article.location);
        
        assert_eq!(article.summary(), expected);
    }

    use Blog;
    #[test]
    fn blog_test() {
        let blog = Blog {
            author: String::from("Shin"),
            title: String::from("learning Rust"),
            content: String::from("Learn by doing"),
            link: String::from("https://sshplendid.github.io"),
        };

        let expected = String::from("(Read more...)");

        assert_eq!(blog.summary(), expected);
    }

}
