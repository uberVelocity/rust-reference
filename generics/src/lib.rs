pub trait Summary {
    fn summarize_author(&self) -> String {
        format!("Random author")
    }

    // Without {}, no default is initialized.
    fn summarize(&self) -> String {
        String::from("(Read more from ...)", self.summarize_author())
    }
}

pub struct NewsJournal {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Default implementation will be used when calling summary
// on NewsJournal object
impl Summary for NewsJournal {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> {
        format("{}", self.author)
    }
}

pub fn tweet() {
    let tweet = Tweet {
        username: String::from("Jess"),
        content: String::from("Dear all, thank you for understanding ..."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

pub fn tweet_message(message: &str) -> String {
    let tweet = Tweet {
        username: String::from("Jess"),
        content: message.to_string(),
        reply: false,
        retweet: false,
    };

    format!("{}: {}", tweet.username, tweet.content)
}

pub fn new_journal() {
    let journal = NewsJournal {
        headline: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New journal available! {}", journal.summarize());
}

pub fn new_article() {
    let article = NewsArticle {
        headline: String::from("Lions are made to sleep"),
        location: String::from("Savanah"),
        author: String::from("Jackie Sally"),
        content: String::from("Some lions mauled some apples."),
    }

    println!("New Article available! {}", article.summarize());
}