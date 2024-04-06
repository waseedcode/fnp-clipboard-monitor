use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::thread;
use std::process::exit;
use std::time::Duration;
use home::home_dir;

fn main() {
    println!("Starting the clipboard monitor...");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut replacement_contents: String = String::new();
    let token = get_token();

    loop {
        let clipboard_content = ctx.get_contents().unwrap();
        if clipboard_content.contains("https://fearnopeer.com/torrents/download/") && clipboard_content != replacement_contents {
            replacement_contents = String::from(format!("{}.{}", &clipboard_content, &token).replace("torrents", "torrent"));

            ctx.set_contents(replacement_contents.to_owned()).unwrap();
            println!("Clipboard updated.");
        }

        // Sleep for a while to avoid high CPU usage
        thread::sleep(Duration::from_secs(1));
    }
}

fn get_token() -> String {
    let mut token = String::new();
    let mut token_file = home_dir().unwrap();
    token_file.push(".fnp_rss_token");

    if token_file.exists() {
        token = std::fs::read_to_string(token_file).unwrap().trim().to_string();
    } else {
        println!("Token file not found. Please create a file named '.fnp_rss_token' in your home directory and paste your RSS token in it.");
        exit(1);
    }
    token
}
