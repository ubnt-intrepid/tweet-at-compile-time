#![feature(plugin)]
#![plugin(tweet_at_compile_time)]

fn main() {
    tweet!("日本語テスト\n🍣食べたい");
}
