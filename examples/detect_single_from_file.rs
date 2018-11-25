/// read a file of 60 strings and find the one that has been 
/// XORd with a single byte
use std::fs::File;
use std::io::prelude::*;

extern crate pals;

fn main() {
    let mut f = File::open("data/4.txt").expect("Should open a file");

    let mut lines = String::new();
    f.read_to_string(&mut lines)
        .expect("error reading contents of file");

    // store all scores from each line
    let mut scores: Vec<pals::LineScore> = Vec::new();
    for l in lines.lines() {
        let line_string = l.to_string();
        let score = pals::LineScore {
            fscore: pals::xor::single_byte(l),
            line: line_string
        };
        scores.push(score);
    }
    let mut min_score = pals::LineScore {
        line: String::from("Nada"),
        fscore: pals::FreqScore {id: 0, score: 10.0}
    };

    for score in scores.iter() {
        if score.fscore.score <= min_score.fscore.score {
            min_score.fscore.score = score.fscore.score;
            min_score.fscore.id = score.fscore.id;
            min_score.line = score.line;
        }
    }

    println!("Min score: {:?} for char: {:?} on line {:?}", min_score.fscore.score, min_score.fscore.id, min_score.line);
    assert_eq!(0, 0);
}
