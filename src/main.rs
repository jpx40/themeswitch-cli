use serde::Deserialize;
mod cli;
mod conf;
mod template;
mod theme;
mod wallpaper;

#[derive(Deserialize, Debug)]
struct Wallpaper {
    name: String,
}
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
    let test: Wallpaper = ron::from_str(
        r#" Wallpaper (
  name: "test",
)"#,
    )
    .unwrap_or_else(|err| panic!("{}", err.to_string()));
    println!("{test:#?}")
}

const _CHECK_OS: () = if cfg!(all(
    not(target_os = "linux"),
    not(feature = "unsupported-os")
)) {
    panic!("Sorry, only Linux is currently supported.");
};
