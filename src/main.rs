use clap::{App, AppSettings, Arg};
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

fn main() {
    let matches = App::new("xargh")
        .setting(AppSettings::TrailingVarArg)
        .version("0.1.0")
        .author("L3viathan <git@l3vi.de>")
        .about("simple xargs replacement")
        .arg(
            Arg::with_name("replace")
                .takes_value(true)
                .long("replace")
                .short("r")
                .help("Replace this with the input"),
        )
        .arg(
            Arg::with_name("shell")
                .short("s")
                .long("shell")
                .help("Whether to shell out (so pipes etc. work)"),
        )
        .arg(
            Arg::with_name("command")
                .takes_value(true)
                .allow_hyphen_values(true)
                .multiple(true)
                .min_values(1)
                .required(true)
                .help("The command to run per line of stdin"),
        )
        .get_matches();
    let vals: Vec<&str> = matches.values_of("command").unwrap().collect();
    let program = vals[0];
    let mut arguments = Vec::new();
    for (i, item) in vals.iter().enumerate() {
        if i != 0 {
            arguments.push(item);
        }
    }
    let replace = match matches.value_of("replace") {
        Some(value) => value,
        None => "",
    };
    if !replace.is_empty() {
        let re = matches.value_of("replace").unwrap();
        for line in io::stdin().lock().lines() {
            let text = line.unwrap();
            if matches.is_present("shell") {
                Command::new("sh")
                    .arg("-c")
                    .arg(program.replace(re, &text))
                    .args(arguments.iter().map(|x| x.replace(re, &text)))
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed");
            } else {
                Command::new(program)
                    .args(arguments.iter().map(|x| x.replace(re, &text)))
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed");
            }
        }
    } else {
        for line in io::stdin().lock().lines() {
            let text = line.unwrap();
            if matches.is_present("shell") {
                Command::new("sh")
                    .arg("-c")
                    .arg(program)
                    .args(&arguments)
                    .arg(text)
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed");
            } else {
                Command::new(program)
                    .args(&arguments)
                    .arg(text)
                    .stdin(Stdio::null())
                    .status()
                    .expect("Failed");
            }
        }
    }
}
