#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(ss =>
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg content: "sort content")
            (@arg input: -i --input "read from stdin")
        ).get_matches();

    if let Some(s) = matches.value_of("content") {
        let mut s = s.chars().collect::<Vec<_>>();
        s.sort();
        println!("{}", s.iter().collect::<String>().trim());
    } else if matches.is_present("input") {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Can't readline");
        let mut s = buf.chars().collect::<Vec<_>>();
        s.sort();
        println!("{}", s.iter().collect::<String>().trim())
    }
}
