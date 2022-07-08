use std::io::{stdin, stdout, Read, Write};
use std::str;

// Buffer sizes
const MAX_CALL: usize = 10;
const MAX_BUFF: usize = 4 * 1024;

// Messages
const COMMANDS_MSG: &str =
    "E: Exit, H: Help, L: List Messages, M: My Details, Q: Set Home Mailbox, S: Compose Message\n";
const WELCOME_MSG: &str = "You have connected to M0ZAH Mailbox\nMSGZAH Version: 0.1";
const USER_PROMPT: &str = ">>> ";
const UNKNOWN_PROMPT: &str = "?";
const COMPOSE_MSG: &str = "Please enter your message and use /e to finish\n";
const TO_MSG: &str = "TO: ";
const HOME_BBS_PROMPT: &str = "Please enter your home BBS/Mailbox\n";

struct Message {
    to: String,
    sender: User,
    date: String,
    text: String,
}
#[derive(Clone)]
struct User {
    callsign: String,
    qth: String,
    total_session_bytes: usize,
}

impl Message {
    fn new() -> Self {
        Self {
            to: String::new(),
            sender: User::new(),
            date: String::new(),
            text: String::new(),
        }
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

fn get_input(esac: Option<u8>, user: &mut User, size: usize) -> [u8; MAX_BUFF] {
    let mut in_buff = [0; MAX_BUFF];
    for (i, c) in stdin().bytes().enumerate() {
        if let Ok(r) = c {
            in_buff[i] = r;
            user.total_session_bytes += i;
        };

        // Testing 0D strip
        if in_buff[i] == 13{
            in_buff[i] = 0;
        }

        if i == MAX_BUFF || i == size {
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

    let mut in_buff = [0; MAX_BUFF];

    if let Ok(bc) = stdin().read(&mut in_buff[..MAX_CALL]) {
        user.total_session_bytes += bc;
    }

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
        in_buff = get_input(None, &mut user, 0);

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
            76 | 108 => {}

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
                in_buff = get_input(Some(10), &mut user, MAX_CALL);
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
                let to = get_input(Some(10), &mut user, MAX_CALL);
                msg.to = match str::from_utf8(&to[..MAX_CALL]) {
                    Ok(v) => v.to_owned().replace('\n', ""),
                    Err(_) => "N0CALL".to_owned(),
                };
                println!("{}", msg.to);
                screen_write(COMPOSE_MSG);
                in_buff = get_input(Some(101), &mut user, MAX_BUFF);
                msg.text = match str::from_utf8(&in_buff[..MAX_BUFF]) {
                    Ok(v) => v.to_owned(),
                    Err(_) => "N0CALL".to_owned(),
                };
                println!(
                    "From: {}\nTo: {}\n{}",
                    msg.sender.callsign, msg.to, msg.text
                );
            }

            _ => {
                screen_write(UNKNOWN_PROMPT);
            }
        }
    }
}
