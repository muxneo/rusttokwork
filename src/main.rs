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
use std::sync::{Arc,Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

const FPS: u32 = 10;


fn main() {

    let (tx, rx): (Sender<Vec<u32>>, Receiver<Vec<u32>>) = mpsc::channel();

    let mut epoch = 0;
    let mut datax : Vec<u32> = Vec::new();

    // twitter inits
    const TRACK: &str = "twitter, facebook, google, travel, art, music, photography, love, fashion, food";
    let track_string = TRACK.split_whitespace().collect::<String>();
    println!("{}",track_string);
    let mut sent_vec : Vec<String> = track_string.split(',').map(String::from).collect::<Vec<String>>();
    let sent_vec_xformatter = sent_vec.clone();
    print!("{:?}", sent_vec);

    let token = Token::new("2sIwehojHI7pvvYv7CTK6QQsx", "RDuVQi9sdSVMM1kZBPUdKIobmVLDJS1Ttjfvhy6g0B46cWP1xG", "1185374845745287168-h9UfqDNcELJDx46B0WTRoBges3IyJl", "UhFYLhOoXZEsiJv6WWf0elB0z5InNr00Hjt4dqhUSwXKF");

    //#[derive(Clone, Copy)]
    let mut counts : BTreeMap<String,u32> = BTreeMap::new();

    let word_regex = Regex::new(r"(?i)[a-z']+").expect("Could not compile regex");

    let stream = TwitterStreamBuilder::filter(token)
        .track(Some(TRACK))
        .listen()
        .unwrap()
        .flatten_stream();




    let child = thread::spawn(move || {
        let sentiment_vec  = vec!["twitter, facebook, google, travel, art, music, photography, love, fashion, food"];
        let bot = stream
            .for_each(move |json| {
                let json = Json::from_str(&json).unwrap();
                let textweet = json.find_path(&["text"]);

                if textweet.is_some() {
                    //println!("is some");
                    let textweet2 = textweet.unwrap();
                    for line in textweet2.as_string() {
                        //let line = line.expect("Error parsing stdin");
                        let words = word_regex.find_iter(&line).map(|(s, e)| &line[s..e]);
                        for mut word in words.map(str::to_lowercase) {
                            if (sent_vec.iter().any(|e| e == &word)) {
                                println!("{}", word);
                                *counts.entry(word.clone()).or_insert(0u32) += 1;

                                let mut idx : u32 = 0;
                                match word.as_ref() {
                                    "twitter" => idx = 0,
                                    "facebook" => idx = 1,
                                    "google" => idx =2,
                                    "travel" => idx = 3,
                                    "art" => idx = 4,
                                    "music" => idx = 5,
                                    "photography" => idx = 6,
                                    "love" => idx = 7,
                                    "fashion" => idx = 8,
                                    "food" => idx = 9,
                                    _ => println!("something else!"),
                                }


                                datax.push(idx as u32);
                                tx.send(datax.clone()).unwrap();
                                //tx.send(&counts).unwrap();
                                //print whole vector
                                for (word, count) in counts.iter() {
                                    println!("###### {} {}", word, count);
                                }
                            }
                        }
                    }
                } else {
                }


                Ok(())
            }).map_err(|e| println!("error: {}", e));
        rt::run(bot);
    });


    let mut window: PistonWindow = WindowSettings::new("Realtime CPU Usage", [1200, 950])
            .samples(4)
            .build()
            .unwrap();

        window.set_max_fps(FPS as u64);

        while let Some(_) = draw_piston_window(&mut window, |b| {
            let root = b.into_drawing_area();


            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .margin(5)
                .caption("Twitter Word Sentiment - by Mrukant", ("Arial", 70.0).into_font())
                .build_ranged(0u32..9u32, 0u32..100u32)?;

            chart
                .configure_mesh()
                .disable_x_mesh()
                .line_style_1(&WHITE.mix(0.3))
                .x_label_offset(60)
                .y_desc("Time")
                .x_desc("Sentiment Words")
                .axis_desc_style(("Arial", 35).into_font())
                .x_label_formatter(&|x| format!("{}", sent_vec_xformatter[*x as usize]))
                .draw()?;

            let dataxx: Vec<u32> = rx.recv().unwrap();

            chart.draw_series(
                Histogram::vertical(&chart)
                    .style(RED.mix(0.5).filled())
                    .data(dataxx.iter().map(|x: &u32| (*x, 1))),
            )?;

            Ok(())


        }){}
}
