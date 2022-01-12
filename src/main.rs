use regex::Regex;
use std::{error::Error, io, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let displays = get_displays()?;
    let stdin = io::stdin();
    print_displays(&displays, |_| true);
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let mut next_skip = buffer.trim().parse::<usize>().unwrap() - 1usize;
    loop {
        let displays = get_displays()?;
        configure_displays(&displays, &mut next_skip)?;
        if next_skip == 0 {
            break;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Display {
    connected: bool,
    id: String,
    resolution: Option<String>,
}

impl Display {
    pub fn from_captures(d: &regex::Captures) -> Display {
        Display {
            id: d.get(1).unwrap().as_str().trim().to_string(),
            connected: d.get(3).map_or(true, |_| false),
            resolution: d
                .get(4)
                .map_or(None, |r| Some(r.as_str().trim().to_string())),
        }
    }
}

fn get_displays() -> Result<Vec<Display>, Box<dyn Error>> {
    let x = String::from_utf8(Command::new("xrandr").output()?.stdout)?;
    let display_regex =
        Regex::new(r"(^DP(-\d){1,2})\s(dis)?connected(\s\d+x\d+\+\d+\+\d+)?")?;
    Ok(x.lines()
        .flat_map(|line| {
            display_regex
                .captures(line)
                .map(|d| Display::from_captures(&d))
        })
        .collect())
}

fn configure_displays(
    displays: &[Display],
    skip: &mut usize,
) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    'displays: for d in displays.iter().skip(*skip) {
        'commands: loop {
            // displays.iter().for_each(|d| loop {
            let mut buffer = String::new();
            println!("{:#?}", d);
            stdin.read_line(&mut buffer).unwrap();
            for c in buffer.trim().chars() {
                match c {
                    'a' | 'b' | 'l' | 'r' => {
                        position(&displays, &d, c);
                    }

                    'c' | 'n' => break 'commands,
                    'd' | 'q' => {
                        *skip = 0;
                        break 'displays;
                    }

                    'e' => {
                        Command::new("xrandr")
                            .arg("--output")
                            .arg(&d.id)
                            .arg("--off")
                            .output()
                            .unwrap();
                        Command::new("xrandr")
                            .arg("--output")
                            .arg(&d.id)
                            .arg("--auto")
                            .output()
                            .unwrap();
                    }

                    'g' => {
                        print_displays(displays, |_| true);
                        let mut buffer = String::new();
                        stdin.read_line(&mut buffer).unwrap();
                        *skip =
                            buffer.trim().parse::<usize>().unwrap() - 1usize;
                        break 'displays;
                    }

                    'o' => {
                        Command::new("xrandr")
                            .arg("--output")
                            .arg(&d.id)
                            .arg("--off")
                            .output()
                            .unwrap();
                    }

                    _ => println!("Unrecognized Option"),
                }
            }
        }
        *skip = 0usize;
    }
    Ok(())
}

fn position(displays: &[Display], d: &Display, pos: char) {
    let stdin = io::stdin();
    print_displays(displays, |(_, d)| d.connected);
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
    let pos = match pos {
        'a' => "--above",
        'b' => "--below",
        'r' => "--right-of",
        'l' => "--left-of",
        _ => unreachable!(),
    };
    let rel: usize = buffer.trim().parse().unwrap();
    let rel = match rel {
        0 => "eDP-1",
        _ => &displays[rel - 1usize].id,
    };
    Command::new("xrandr")
        .arg("--output")
        .arg(&d.id)
        .arg(pos)
        .arg(rel)
        .output()
        .unwrap();
}

fn print_displays<P>(displays: &[Display], pred: P)
where
    P: FnMut(&(usize, &Display)) -> bool,
{
    println!("0: eDP-1");
    (1..)
        .zip(displays.iter())
        .filter(pred)
        .for_each(|(i, d)| println!("{}: {:?}", i, d));
}
