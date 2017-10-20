#![feature(plugin)]
#![plugin(tweet_at_compile_time)]

fn main() {
    tweet!("This tweet was posted from compiler plugin of Rust :)");
}
