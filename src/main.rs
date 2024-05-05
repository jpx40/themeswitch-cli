use serde::Deserialize;
mod api;
mod cli;
mod conf;
mod parser;
mod store;
mod template;
mod theme;
mod util;
mod wallpaper;
use crate::parser::*;
use crate::store::PROFILE;
use pest::Parser;
use pest_derive::Parser;
fn main() {
    //     let w = wallpaper::Wallpaper {
    //         name: "cat_lofi_cafe".to_string(),
    //         path: "~/.config/swww/Tokyo-Night/edger_lucy_neon.jpg".to_string(),
    //         config: None,
    //     };
    //
    //     match w.set_wallpaper() {
    //         Ok(_) => println!("test_set_wallpaper was successfull"),
    //         Err(e) => eprintln!("{e}"),
    //     }
    parser::parse_conf("test.conf");
    //let mut p = PROFILE.lock().unwrap().clone();
}

// const _CHECK_OS: () = if cfg!(all(
//     not(target_os = "linux"),
//     not(feature = "unsupported-os")
// )) {
//     panic!("Sorry, only Linux is currently supported.");
//};
