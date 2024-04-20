mod cli;
mod conf;
mod template;
mod theme;
mod wallpaper;

fn main() {
    let w = wallpaper::Wallpaper {
        name: "cat_lofi_cafe".to_string(),
        path: "~/.config/swww/Tokyo-Night/edger_lucy_neon.jpg".to_string(),
        config: None,
    };

    match w.set_wallpaper() {
        Ok(_) => println!("test_set_wallpaper was successfull"),
        Err(e) => eprintln!("{e}"),
    }
}
