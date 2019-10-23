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

const FPS: u32 = 10;

//#[derive(Debug, Default)]
pub struct A<'a>{
    json : &'a str,
    window : PistonWindow,
}


fn main() {
    // plotting histogram inits
//    let mut var1 = A{
//      json : "",
//        window : WindowSettings::new("Realtime CPU Usage", [450, 300])
//            .samples(4)
//            .build()
//            .unwrap(),
//    };
    let mut window: PistonWindow = WindowSettings::new("Realtime CPU Usage", [450, 300])
        .samples(4)
        .build()
        .unwrap();

    window.set_max_fps(FPS as u64);
    let mut epoch = 0;
    let mut data : Vec<u32> = vec![];

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





//    let rest = tweetust::TwitterClient::new(
//        token,
//        tweetust::DefaultHttpHandler::with_https_connector().unwrap(),
//    );
    let winsafe = Arc::new(Mutex::new(window));


    let bot = stream
        .for_each(move |json| {
//            let mut var1 = A{
//                json : "",
//                window : WindowSettings::new("Realtime CPU Usage", [450, 300])
//                    .samples(4)
//                    .build()
//                    .unwrap(),
//            };

            let json = Json::from_str(&json).unwrap();
            let textweet = json.find_path(&["text"]);
            //if tweettext2 != None {
            //    println!("{}",tweettext2);
            //}
            //else {  }

            //match textweet {
            //       Some(x) => println!("{}",x),
            //        None => println!("None"),
            //}

            if textweet.is_some(){
                //println!("is some");
                let textweet2 = textweet.unwrap();
                for line in textweet2.as_string(){
                    //let line = line.expect("Error parsing stdin");
                    let words = word_regex.find_iter(&line).map(|(s, e)| &line[s..e]);
                    for word in words.map(str::to_lowercase) {
                        if(sent_vec.iter().any(|e| e == &word)){
                            println!("{}",word);
                            *counts.entry(word).or_insert(0u32) += 1;


                            let winsafe_ref = Arc::clone(&winsafe);
                            let mut win = winsafe_ref.lock().unwrap();



                            let event: _ = draw_piston_window(&mut win, |b| {
                                let root = b.into_drawing_area();


                                root.fill(&WHITE)?;

                                let mut chart = ChartBuilder::on(&root)
                                    .x_label_area_size(35)
                                    .y_label_area_size(40)
                                    .margin(5)
                                    .caption("Histogram Test", ("Arial", 50.0).into_font())
                                    .build_ranged(0u32..10u32, 0u32..10u32)?;

                                chart
                                    .configure_mesh()
                                    .disable_x_mesh()
                                    .line_style_1(&WHITE.mix(0.3))
                                    .x_label_offset(30)
                                    .y_desc("Count")
                                    .x_desc("Bucket")
                                    .axis_desc_style(("Arial", 15).into_font())
                                    .draw()?;

                                let data = [
                                    0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
                                ];

                                chart.draw_series(
                                    Histogram::vertical(&chart)
                                        .style(RED.mix(0.5).filled())
                                        .data(data.iter().map(|x: &u32| (*x, 1))),
                                )?;

                                Ok(())


                            }).unwrap();



                            //print whole vector
//                            for (word, count) in counts.iter() {
//                                println!("###### {} {}", word, count);
//                            }
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
