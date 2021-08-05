use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use scraper::{ Html, Selector };
use whoami;

struct Version {
    major: u32,
    minor: u32,
    patch: u32
}

fn exract_julia_version(input: &str) -> Option<Version> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Julia[\s-]*(?P<major>\d+).(?P<minor>\d+).(?P<patch>\d+)").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        let major = cap.name("major").map(|major| major.as_str());
        let minor = cap.name("minor").map(|minor| minor.as_str());
        let patch = cap.name("patch").map(|patch| patch.as_str());
        Some(Version {
            major: major.unwrap().parse::<u32>().unwrap(),
            minor: minor.unwrap().parse::<u32>().unwrap(),
            patch: patch.unwrap().parse::<u32>().unwrap(),
        })
    })
}

fn latest_julia() {
    let resp = reqwest::blocking::get("https://julialang.org/downloads/#current_stable_release").expect("Unable to retrieve the latest version of Julia");
    let body = resp.text().unwrap();
    let fragment = Html::parse_document(&body);
    let latest = Selector::parse("#current_stable_release").unwrap();
    for item in fragment.select(&latest) {
        println!("{}", item.text().collect::<Vec<_>>()[0]);
    }
}

pub fn julia_ver() {
    let mut vers: Vec<Version> = vec![];
    for dir in fs::read_dir(format!("C:\\Users\\{}\\AppData\\Local\\Programs", whoami::username())).expect("Unable to find local Julia installation") {
        if let Some(ver) = exract_julia_version(dir.unwrap().path().to_str().expect("a str")) {
            vers.push(ver);
        }
    }
    let max_major = vers.iter().max_by_key(|v| v.major);
    let max_major_minor = max_major.iter().filter(|v| v.major == max_major.unwrap().major).max_by_key(|v| v.minor);
    println!(
        "Your Julia version: v{}.{}.{}",
        max_major_minor.unwrap().major, max_major_minor.unwrap().minor, max_major_minor.unwrap().patch
    );
    latest_julia()
}