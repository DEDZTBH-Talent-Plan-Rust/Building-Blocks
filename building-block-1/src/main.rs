#[macro_use]
extern crate clap;

use clap::App;

extern crate dotenv_codegen;

use dotenv_codegen::dotenv;

use std::env;
use std::fmt;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    match m.value_of("config") {
        None => { println!("Gimme a config!"); }
        Some(a) => { println!("Config: {}", a); }
    }

    println!("PORT: {}", dotenv!("PORT"));

    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key)
    }
}

enum MyErr {
    Reason1(String),
    Reason2(String, u32),
}

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyErr::Reason1(ref s) =>
                write!(f, "`{}` is the error", s),
            MyErr::Reason2(ref s, ref num) =>
                write!(f, "`{}` and `{}` are error", s, num),
        }
    }
}