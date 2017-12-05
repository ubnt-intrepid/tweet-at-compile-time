#![feature(proc_macro)]

#[macro_use]
extern crate tweet_at_compile_time;

use tweet_at_compile_time::tweet;

fn main() {
    tweet! {
        token: "token.json",
        body: "This tweet was posted from compiler plugin of Rust :)",
    }
}
