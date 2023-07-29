#![allow(unused)]
use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Page {
    pub author: String,
    pub content: String,
}

impl Summary for Page {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub fn notify(item: &(impl Summary + Display)) {
    // The item must implement both Summary and Display
    println!("{}", item.summarize());
}

pub fn notify_diff(item1: &impl Summary, item2: &impl Summary) {
    // The items can be of different type, as long as it implements Summary
    println!("item1: {}", item1.summarize());
    println!("item2: {}", item2.summarize());
}

pub fn notify_same<T: Summary>(item1: &T, item2: &T) {
    // The items must be of the same type
    println!("item1: {}", item1.summarize());
    println!("item2: {}", item2.summarize());
}

// This is not the way
// pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> u8 {
//     println!("{} {:#?}", t.to_owned(), u);
//     42
// }

pub fn some_function<T, U>(t: &T, u: &U) -> u8
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("{} {:#?}", t.to_owned(), u);
    42
}

pub fn returns_summarizable() -> impl Summary {
    Page {
        author: "author".to_owned(),
        content: "content".to_owned(),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x >= y");
        } else {
            println!("x < y");
        }
    }

    fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize_for_news_article() {
        let article = NewsArticle {
            headline: "headline".to_owned(),
            location: "location".to_owned(),
            author: "author".to_owned(),
            content: "content".to_owned(),
        };
        assert_eq!(article.summarize(), "headline, by author (location)");
    }

    #[test]
    fn test_summarize_for_tweet() {
        let tweet = Tweet {
            username: "username".to_owned(),
            content: "content".to_owned(),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), "username: content");
    }

    #[test]
    fn test_summarize_for_page() {
        let page = Page {
            author: "author".to_owned(),
            content: "content".to_owned(),
        };
        assert_eq!(page.summarize(), "Read more from author...");
    }

    #[test]
    fn test_returns_summarizable() {
        let page = Page {
            author: "author".to_owned(),
            content: "content".to_owned(),
        };
        assert_eq!(page.summarize(), returns_summarizable().summarize())
    }

    #[test]
    fn test_pair_new() {
        let pair = Pair::new(69, 420);
        assert_eq!(pair.x, 69);
        assert_eq!(pair.y, 420);
    }
}
