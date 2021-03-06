use chrono::prelude::*;
use std::fs::{self, File};
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::str;

// Offsets
const LIST_SEL: usize = 1;

// Buffer sizes
const MAX_CALL: usize = 10;
const MAX_BUFF: usize = 4 * 1024;

// Admin
const ADMIN: &str = "M0ZAH";

// Messages
const COMMANDS_MSG: &str =
    "E: Exit, H: Help, L: List Messages, M: My Details, Q: Set Home Mailbox, S: Compose Message\n";
const WELCOME_MSG: &str = "You have connected to M0ZAH Mailbox\nMSGZAH Version: 0.1";
const USER_PROMPT: &str = ">>> ";
const UNKNOWN_PROMPT: &str = "?";
const COMPOSE_MSG: &str = "Please enter your message and use /e to finish\n";
const TO_MSG: &str = "TO: ";
const HOME_BBS_PROMPT: &str = "Please enter your home BBS/Mailbox\n";
const MESSAGE_SELECT_PROMPT: &str = "Please enter the message number you'd like to read!\n";
const DELETE_MSG_PROMPT: &str = "Would you like to delete this message? N/Y: ";

enum EventType {
    Raw,
    Capital,
    Decimal,
}

struct Message {
    to: String,
    sender: User,
    date: chrono::DateTime<chrono::Utc>,
    messages: Vec<std::path::PathBuf>,
    text: String,
}

#[derive(Clone)]
struct User {
    callsign: String,
    qth: String,
    total_session_bytes: usize,
}

impl Message {
    fn del(self, msg_num: usize) {
        match fs::remove_file(&self.messages[msg_num]) {
            Ok(_) => {
                println!("Msg [{}] deleted", msg_num)
            }
            Err(e) => {
                println!("Delete error: {}", e);
            }
        }
    }

    fn new() -> Self {
        Self {
            to: String::new(),
            sender: User::new(),
            date: Utc::now(),
            messages: std::vec::Vec::new(),
            text: String::new(),
        }
    }

    fn load(&mut self) {
        if let Ok(message_location) = fs::read_dir("./store") {
            for (_, m) in message_location.enumerate() {
                let p = m.unwrap().path();
                self.messages.push(p);
            }
        }
    }

    fn save(self) {
        let path_construct = format!(
            "./store/{}-{}.dat",
            self.date,
            self.sender.callsign.replace('\0', "").replace('\n', "")
        );
        let path = Path::new(&path_construct);
        if let Ok(mut f) = File::create(path) {
            match f.write_all(self.text.as_bytes()) {
                Err(e) => {
                    println!("Error: {}", e)
                }
                Ok(_) => {
                    println!("Saved!");
                }
            }
        }
    }

    fn show(&mut self, msg_num: usize) {
        let mut d_buff = [0; MAX_BUFF];
        if let Ok(mut f) = File::open(self.messages[msg_num].as_path()) {
            match f.read(&mut d_buff[..]) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
            self.text = match str::from_utf8(&d_buff[..MAX_BUFF]) {
                Ok(v) => v.to_owned(),
                Err(_) => "N0CALL".to_owned(),
            };
        }
        println!("{}", self.text);
    }
}

impl User {
    fn new() -> Self {
        Self {
            callsign: String::new(),
            qth: String::from("N0HOME"),
            total_session_bytes: 0,
        }
    }
}

fn get_input(event: EventType, esac: Option<u8>, user: &mut User, size: usize) -> [u8; MAX_BUFF] {
    let mut in_buff = [0; MAX_BUFF];
    for (i, c) in stdin().bytes().enumerate() {
        if let Ok(mut r) = c {
            match event {
                EventType::Raw => {
                    // Do nothing,
                }
                EventType::Capital => {
                    if (97..=122).contains(&r) {
                        r -= 32;
                    }
                }
                EventType::Decimal => {
                    if (48..=57).contains(&r) {
                        r -= 48;
                    } else {
                        r = 0;
                    }
                }
            }
            in_buff[i] = r;
            user.total_session_bytes += i + 1;
        };

        // Testing 0D strip
        if in_buff[i] == 13 {
            in_buff[i] = 0;
        }

        if i == MAX_BUFF - 1 || i == size {
            return in_buff;
        }

        match esac {
            Some(es) => {
                if i > 1 {
                    if es == 10 && in_buff[i] == es {
                        return in_buff;
                    }
                    match in_buff.get(i - 1) {
                        Some(v) => {
                            if v == &47 && in_buff[i] == es {
                                println!("Received: {} Bytes", i);
                                in_buff[i] = 0;
                                in_buff[i - 1] = 0;
                                return in_buff;
                            }
                        }
                        None => {}
                    }
                }
            }
            None => continue,
        }
    }
    in_buff
}

fn screen_write(b: &str) {
    stdout().write_all(b.as_bytes()).unwrap();
    stdout().flush().unwrap();
}

fn main() {
    let mut user: User = User::new();

    let mut in_buff = get_input(EventType::Raw, Some(10), &mut user, MAX_CALL);
    user.callsign = match str::from_utf8(&in_buff[..MAX_CALL]) {
        Ok(v) => v.to_owned(),
        Err(_) => "N0CALL".to_owned(),
    };

    let welcome = format!(
        "Welcome: {}{}\n\n{}\n",
        user.callsign, WELCOME_MSG, COMMANDS_MSG
    );

    screen_write(&welcome);

    loop {
        in_buff = get_input(EventType::Raw, None, &mut user, 0);

        match in_buff[0] {
            0 => {
                // Ignore 0
            }

            10 => {
                screen_write(USER_PROMPT);
            }

            // Exit
            69 | 101 => return,

            // Help
            72 | 104 => {
                screen_write(COMMANDS_MSG);
            }

            // List Messages
            76 | 108 => {
                let mut msg: Message = Message::new();
                msg.load();
                if msg.messages.is_empty() {
                    continue;
                }
                for (i, p) in msg.messages.iter().enumerate() {
                    println!("Message: [{}] -> {}", i, p.display());
                }
                screen_write(MESSAGE_SELECT_PROMPT);
                in_buff = get_input(EventType::Decimal, Some(0x0A), &mut user, 2);
                let sel = in_buff[LIST_SEL] as usize * 10 + in_buff[LIST_SEL + 1] as usize;
                if sel >= msg.messages.len() || !(0..=99).contains(&sel) {
                    continue;
                }
                msg.show(sel);

                if user.callsign.contains(ADMIN) {
                    screen_write(DELETE_MSG_PROMPT);
                    in_buff = get_input(EventType::Capital, Some(0x0A), &mut user, 0);
                    if in_buff[0] == 89 {
                        msg.del(sel);
                    }
                }
            }

            // List My Details
            77 | 109 => {
                let details = format!(
                    "Callsign: {}\nHome Mailbox: {}\n\nTotal Bytes Received: {}\n",
                    user.callsign, user.qth, user.total_session_bytes
                );
                screen_write(&details);
            }

            // Set Home Mailbox
            81 | 113 => {
                screen_write(HOME_BBS_PROMPT);
                in_buff = get_input(EventType::Raw, Some(10), &mut user, MAX_CALL);
                user.qth = match str::from_utf8(&in_buff[..MAX_CALL]) {
                    Ok(v) => v.to_owned().replace('\n', ""),
                    Err(_) => "N0CALL".to_owned(),
                };
                println!("HomeBBS: {}", user.qth);
            }

            // Send msg
            83 | 115 => {
                let mut msg: Message = Message::new();
                msg.sender = user.clone();
                screen_write(TO_MSG);
                let to = get_input(EventType::Raw, Some(10), &mut user, MAX_CALL);
                msg.to = match str::from_utf8(&to[..MAX_CALL]) {
                    Ok(v) => v.to_owned().replace('\n', ""),
                    Err(_) => "N0CALL".to_owned(),
                };
                println!("{}", msg.to);
                screen_write(COMPOSE_MSG);
                msg.date = Utc::now();
                in_buff = get_input(EventType::Raw, Some(101), &mut user, MAX_BUFF);
                msg.text = match str::from_utf8(&in_buff[..MAX_BUFF]) {
                    Ok(v) => v.to_owned(),
                    Err(_) => "N0CALL".to_owned(),
                };
                println!(
                    "From: {}\nTo: {}\n{}",
                    msg.sender.callsign, msg.to, msg.text
                );
                msg.save();
            }

            _ => {
                screen_write(UNKNOWN_PROMPT);
            }
        }
    }
}
