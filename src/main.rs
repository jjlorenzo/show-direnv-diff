
use base64;
use colored::*;
use flate2::read::ZlibDecoder;
use json;
use json::array;
use json::object;
use std::env;
use std::io::Read;
use std::panic;
use structopt::clap::arg_enum;
use structopt::StructOpt;


#[derive(Debug)]
#[derive(StructOpt)]
struct ArgumentParser {

  #[structopt(
    case_insensitive = true,
    default_value = &Output::variants()[0],
    help = "Output format",
    possible_values = &Output::variants(),
    short,
  )]
  fmt: Output,
}


arg_enum! {
  #[allow(non_camel_case_types)]
  #[derive(Debug)]
  #[derive(Eq)]
  #[derive(PartialEq)]
  enum Output { direnv, json }
}


fn main() {
  let opts = ArgumentParser::from_args();

  panic::set_hook(Box::new(|_| {}));

  let base64_value = env::var("DIRENV_DIFF").unwrap();
  let zlib_value = base64::decode_config(&base64_value, base64::URL_SAFE).unwrap();
  let mut zlib_decoder = ZlibDecoder::new(&zlib_value[..]);
  let mut json_value = String::new();
  zlib_decoder.read_to_string(&mut json_value).unwrap();
  let data = json::parse(&json_value).unwrap();

  if opts.fmt == Output::direnv {
    let log_format = "==>";
    for arr in [["p", "Before"], ["n", "After"]].iter() {
      println!("{}", format!("{} Environment {}", log_format, arr[1]).green());
      for (key, val) in data[arr[0]].entries() {
        if ! key.starts_with("DIRENV_") {
          if key == "PATH" {
            for p in val.as_str().unwrap().split(":") {
              println!("{}", format!("{}   PATH={}", log_format, p).yellow());
            }
          }
          else {
            println!("{}", format!("{}   {}={}", log_format, key, val).yellow());
          }
        }
      }
    }
  }
  else {
    let mut resp = object!{"before" => object!{}, "after" => object!{}};
    for arr in [["p", "before"], ["n", "after"]].iter() {
      for (key, val) in data[arr[0]].entries() {
        if ! key.starts_with("DIRENV_") {
          if key == "PATH" {
            resp[arr[1]][key] = array![val.as_str().unwrap().split(":").collect::<Vec<&str>>()];
          }
          else {
            resp[arr[1]][key] = val.clone();
          }
        }
      }
    }
    println!("{}", json::stringify_pretty(resp, 2));
  }
}
