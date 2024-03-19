use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Teppei Kitagawa <tkitagaw@student.42tokyo.jp>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text to echo")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("no_newline")
                .long("no-newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let no_newline = matches.is_present("no_newline");
    print!("{}{}", text.join(" "), if no_newline { "" } else { "\n" });
}
