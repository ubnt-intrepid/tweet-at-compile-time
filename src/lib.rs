#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate egg_mode;
extern crate tokio_core;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use proc_macro::{TokenStream, TokenTree, TokenNode, Literal};

use std::fs::File;
use egg_mode::{KeyPair, Token};
use egg_mode::tweet::DraftTweet;
use tokio_core::reactor::Core;

#[derive(Debug)]
struct Tweet {
    source_file: String,
    token: String,
    body: String,
}

impl Tweet {
    fn from_tokens(tokens: TokenStream) -> Tweet {
        let mut tokens = tokens.into_iter().peekable();

        // At first, read the source path of call site...
        let source_file;
        if let Some(&TokenTree { ref span, .. }) = tokens.peek() {
            source_file = span.source_file().as_str().to_string();
        } else {
            panic!("could not determine source path of call site");
        }

        fn from_string_literal(value: Literal) -> String {
            value.to_string()
                .trim_left_matches("\"")
                .trim_right_matches("\"")
                .to_string()
        }

        // Next, parse the token stream and get keys.
        let mut token = None;
        let mut body = None;
        loop {
            let key = tokens.next();
            let colon = tokens.next();
            let value = tokens.next();
            let comma = tokens.next();
            match (key, colon, value, comma) {
                (Some(TokenTree { kind: TokenNode::Term(key), .. }),
                 Some(TokenTree { kind: TokenNode::Op(':', ..), .. }),
                 Some(TokenTree { kind: TokenNode::Literal(value), .. }),
                 Some(TokenTree { kind: TokenNode::Op(',', ..), .. })) => {
                     // check if 'value' is a string literal.
                     match key.as_str() {
                         "token" => token = Some(from_string_literal(value)),
                         "body" => body = Some(from_string_literal(value)),
                         _ => {}
                     }
                }
                _ => break,
            }
        }

        Tweet {
            source_file,
            token: token.unwrap(),
            body: body.unwrap(),
        }
    }

    fn get_access_token(&self) -> Token {
        use std::path::Path;
        let token_path = Path::new(&self.source_file).parent().unwrap().join(&self.token);
        let reader = File::open(&token_path).unwrap();

        #[derive(Deserialize)]
        struct TokenData {
            consumer_key: String,
            consumer_secret: String,
            access_token: String,
            access_secret: String,
        }
        let TokenData {
            consumer_key,
            consumer_secret,
            access_token,
            access_secret,
        } = serde_json::from_reader(reader).unwrap();

        Token::Access {
            consumer: KeyPair::new(consumer_key, consumer_secret),
            access: KeyPair::new(access_token, access_secret),
        }
    }

    fn do_tweet(&self) {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let token = self.get_access_token();
        let task = DraftTweet::new(&self.body).send(&token, &handle);

        core.run(task).unwrap();
    }
}

#[proc_macro]
pub fn tweet(input: TokenStream) -> TokenStream {
    let tweet = Tweet::from_tokens(input);
    tweet.do_tweet();
    "".parse().unwrap()
}