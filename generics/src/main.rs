mod generics;
mod lib;

fn main() {
    lib::tweet();
    let received_tweet = lib::tweet_message(&String::from(" the thought process presented today..."));
    println!("received_tweet: {}", received_tweet);

    lib::new_article();
}
