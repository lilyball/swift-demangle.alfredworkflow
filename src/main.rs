#![feature(exit_status)]

extern crate alfred;

use std::env;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() {
    let symbol = env::args().skip(1).next().expect("expected argument");
    let output = match Command::new("xcrun").args(&["swift-demangle", "-compact", &symbol])
                                            .stdin(Stdio::null()).stderr(Stdio::inherit())
                                            .output() {
        Ok(out) => if out.status.success() {
            first_line(String::from_utf8(out.stdout).unwrap())
        } else {
            let _ = writeln!(&mut io::stderr(), "swift-demangle exited with {}", out.status);
            env::set_exit_status(1);
            return;
        },
        Err(err) => {
            let _ = writeln!(&mut io::stderr(), "I/O error: {}", err);
            env::set_exit_status(1);
            return;
        }
    };
    if let Err(err) = alfred::write_items(io::stdout(), &[
        alfred::ItemBuilder::new(format!("Demangle Swift Symbol '{}'", symbol))
                            .subtitle(output).into_item()
    ]) {
        let _ = writeln!(&mut io::stderr(), "I/O error: {}", err);
        env::set_exit_status(1);
        return;
    }
}

fn first_line(mut s: String) -> String {
    if let Some(idx) = s.find('\n') {
        s.truncate(idx);
    }
    s
}
