#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
#[macro_use(panictry)]
extern crate syntax;

extern crate aurelius;
extern crate egg_mode;
extern crate futures;
extern crate tokio_core;

use syntax::ext::base::{self, DummyResult, ExtCtxt, MacResult};
use syntax::ext::quote::rt::Span;
use syntax::errors;
use syntax::parse::token;
use syntax::tokenstream::TokenTree;
use rustc_plugin::Registry;

use std::io;
use egg_mode::KeyPair;
use egg_mode::tweet::DraftTweet;
use aurelius::browser;
use tokio_core::reactor::Core;


const CONSUMER_KEY: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/consumer_key"));
const CONSUMER_SECRET: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/consumer_secret"));


fn from_console(msg: &str) -> String {
    println!("{}", msg);
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    line.trim().to_string()
}

fn do_tweet(
    consumer_key: &'static str,
    consumer_secret: &'static str,
    tweet: &str,
) -> Result<(), egg_mode::error::Error> {
    let mut core = Core::new()?;
    let handle = core.handle();

    // get access token
    let consumer = KeyPair::new(consumer_key, consumer_secret);
    let request_token = core.run(egg_mode::request_token(&consumer, "oob", &handle))?;
    let auth_url = egg_mode::authorize_url(&request_token);
    let _ = browser::open(&auth_url);
    let pin = from_console("Put your PIN");
    if pin.is_empty() {
        return Ok(());
    }
    let (token, _user_id, _screen_name) = core.run(egg_mode::access_token(
        consumer,
        &request_token,
        pin.as_str(),
        &handle,
    ))?;

    core.run(DraftTweet::new(tweet).send(&token, &handle))?;

    Ok(())
}

fn tweet(ctx: &mut ExtCtxt, span: Span, args: &[TokenTree]) -> Box<MacResult> {
    let mut parser = ctx.new_parser_from_tts(args);
    if parser.token == token::Eof {
        ctx.span_err(span, "requires a format string argument");
        return DummyResult::expr(span);
    }

    // Extract tweet string
    let tweet_expr = panictry!(parser.parse_expr());
    let tweet_str = match base::expr_to_spanned_string(
        ctx,
        tweet_expr,
        "The argument must be a string literal.",
    ) {
        Some(fmt) => fmt,
        None => return DummyResult::expr(span),
    };
    let tweet = &*tweet_str.node.0.as_str();
    if tweet.chars().count() > 140 {
        ctx.span_err(span, "The length of tweet must be less or equal to 140.");
        return DummyResult::expr(span);
    }

    // Post tweet
    let consumer_key = CONSUMER_KEY.trim();
    let consumer_secret = CONSUMER_SECRET.trim();
    if let Err(err) = do_tweet(consumer_key, consumer_secret, tweet) {
        ctx.span_err(span, &format!("Unknown error: {:?}", err));
        return DummyResult::expr(span);
    }

    DummyResult::any(span)
}

#[plugin_registrar]
pub fn register_plugin(registry: &mut Registry) {
    registry.register_macro("tweet", tweet);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
