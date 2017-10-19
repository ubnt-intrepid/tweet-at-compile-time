#![feature(plugin)]
#![plugin(tweet_at_compile_time)]

fn main() {
    tweet!("Test");
}
