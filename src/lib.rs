#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

extern crate egg_mode;
extern crate futures;
extern crate tokio_core;

use syntax::ext::base::{self, DummyResult, ExtCtxt, MacResult};
use syntax::ext::quote::rt::Span;
use syntax::tokenstream::TokenTree;
use rustc_plugin::Registry;

use egg_mode::{KeyPair, Token};
use egg_mode::tweet::DraftTweet;
use tokio_core::reactor::Core;


fn do_tweet(tweet: &str) -> Result<(), egg_mode::error::Error> {
    // TODO: read consumer/access token from configuration file
    let consumer_key =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/keys/consumer_key")).trim();
    let consumer_secret =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/keys/consumer_secret")).trim();
    let access_token =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/keys/access_token")).trim();
    let access_secret =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/keys/access_secret")).trim();
    let token = Token::Access {
        consumer: KeyPair::new(consumer_key, consumer_secret),
        access: KeyPair::new(access_token, access_secret),
    };

    let mut core = Core::new()?;
    let handle = core.handle();
    core.run(DraftTweet::new(tweet).send(&token, &handle))?;

    Ok(())
}

fn tweet(ctx: &mut ExtCtxt, span: Span, args: &[TokenTree]) -> Box<MacResult> {
    let tweet = match base::get_single_str_from_tts(ctx, span, args, "tweet!") {
        Some(tweet) => {
            if tweet.chars().count() > 140 {
                ctx.span_err(span, "The length of tweet must be less or equal to 140.");
                return DummyResult::expr(span);
            }
            tweet
        }
        None => return DummyResult::expr(span),
    };

    if let Err(err) = do_tweet(&tweet) {
        ctx.span_err(span, &err.to_string());
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
