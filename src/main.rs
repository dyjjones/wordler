use regex::Regex;
use std::env;
use std::fs;

const WORD_SIZE: usize = 5;

#[derive(Debug)]
enum Rule {
    Exclude(char),
    ExcludeAt(char, usize), // excluded at a position, but included in the word
}

impl Rule {
    fn new(to_exclude: &str) -> Self {
        if to_exclude.len() == 1 {
            Self::Exclude(to_exclude.chars().next().unwrap())
        } else {
            Self::ExcludeAt(
                to_exclude.chars().next().unwrap(),
                to_exclude[2..3].parse::<usize>().unwrap(),
            )
        }
    }
}

// returns true if c in word, but not checking ignore_index
fn check_in(c: char, word: &str, ignore_index: usize) -> bool {
    for (i, ch) in word.chars().enumerate() {
        if i == ignore_index {
            continue;
        }
        if ch == c {
            return true;
        }
    }
    false
}

fn char_filter(word: &str, to_exclude: &Vec<Rule>) -> bool {
    let word_vec = word.chars().collect::<Vec<_>>();
    for ex in to_exclude {
        match ex {
            Rule::Exclude(c) => {
                if word_vec.contains(c) {
                    return false;
                }
            }
            Rule::ExcludeAt(c, i) => {
                // or none of the other letters are that char
                if word_vec[*i] == *c || !check_in(*c, word, *i) {
                    return false;
                }
            }
        }
    }
    true
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // [file_path, regex, exclude_letters] = args
    let contents = fs::read_to_string(&args[1])
        .expect("Cannot read file: {&args[1]}")
        .to_lowercase();
    let contents_filter = contents.split('\n');

    let letters = 'a'..='z';
    let re = Regex::new(&args[2]).unwrap();

    let excluded_chars = if args.len() < 4 || args[3].is_empty() {
        vec![]
    } else {
        args[3].split(',').map(Rule::new).collect::<Vec<_>>()
    };

    let contents_filter = contents_filter
        .filter(|&w| w.len() == WORD_SIZE)
        .filter(|&w| {
            w.chars()
                .map(|c| letters.contains(&c))
                .into_iter()
                .all(|b| b)
        })
        .filter(|&w| re.is_match(w))
        .filter(|&w| char_filter(w, &excluded_chars))
        .collect::<Vec<_>>();

    for word in contents_filter {
        println!("{word}");
    }
}
