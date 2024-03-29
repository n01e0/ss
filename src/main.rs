#[macro_use]
extern crate clap;

use std::io::Read;

fn main() {
    let matches = clap_app!(ss =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg content: "sort content")
            (@arg input: -i --input "read from stdin")
        ).get_matches();

    if let Some(s) = matches.value_of("content") {
        println!("{}", sort(s.into()));
    } else if matches.is_present("input") {
        let mut s = Vec::new();
        std::io::stdin().read_to_end(&mut s).expect("Can't readline");
        println!("{}", sort(String::from_utf8_lossy(&s).into()));
    }
}

fn sort(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.iter().collect::<String>().trim().into()
}
