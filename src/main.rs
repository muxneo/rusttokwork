extern crate serde_json as json;
extern crate tweetust;
extern crate twitter_stream;
extern crate twitter_stream_message;
extern crate rustc_serialize;
extern crate regex;


use std::fs::File;
use std::path::PathBuf;
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::{Error, Token, TwitterStreamBuilder};
use twitter_stream_message::StreamMessage;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use regex::Regex;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use std::collections::vec_deque::VecDeque;


fn main() {
    // twitter inits
    const TRACK: &str = "twitter, facebook, google, travel, art, music, photography, love, fashion, food";
    let track_string = TRACK.split_whitespace().collect::<String>();
    println!("{}",track_string);
    let sent_vec = track_string.split(',').map(String::from).collect::<Vec<String>>();
    print!("{:?}", sent_vec);

    let token = Token::new("2sIwehojHI7pvvYv7CTK6QQsx", "RDuVQi9sdSVMM1kZBPUdKIobmVLDJS1Ttjfvhy6g0B46cWP1xG", "1185374845745287168-h9UfqDNcELJDx46B0WTRoBges3IyJl", "UhFYLhOoXZEsiJv6WWf0elB0z5InNr00Hjt4dqhUSwXKF");

    let mut counts : BTreeMap<String,u32> = BTreeMap::new();

    let word_regex = Regex::new(r"(?i)[a-z']+").expect("Could not compile regex");

    let stream = TwitterStreamBuilder::filter(token)
        .track(Some(TRACK))
        .listen()
        .unwrap()
        .flatten_stream();

    println!("");
    let bot = stream
        .for_each(move |json| {

            let json = Json::from_str(&json).unwrap();
            let textweet = json.find_path(&["text"]);

            if textweet.is_some(){
                //println!("is some");
                let textweet2 = textweet.unwrap();
                for line in textweet2.as_string(){
                    //let line = line.expect("Error parsing stdin");
                    let words = word_regex.find_iter(&line).map(|(s, e)| &line[s..e]);
                    for word in words.map(str::to_lowercase) {
                        if(sent_vec.iter().any(|e| e == &word)){
                            println!("");
                            println!("Tweet = {}",line);
                            println!("Word inside Tweet = {}",word);
                            *counts.entry(word).or_insert(0u32) += 1;

                            println!("Running total");
                            //running total whole vector
                            for (word, count) in counts.iter() {
                                println!("## {} {}", word, count);
                            }
                        }

                    }
                }
            }
            else{
                //println!("NONE")
            }

            Ok(())
        }).map_err(|e| println!("error: {}", e));

    rt::run(bot);
}
