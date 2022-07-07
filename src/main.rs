use std::io::{stdin, stdout, Read, Write};
use std::str;


// Buffer sizes
const MAX_CALL: usize = 8;
const MAX_BUFF: usize = 4 * 1024;

// Messages
const COMMANDS_MSG: &str = "H: Help, S: Compose Message, L: List Messages, E: Exit\n";
const WELCOME_MSG: &str = "You have connected to M0ZAH Mailbox\nMSGZAH Version: 0.1";
const USER_PROMPT: &str = ">>> ";
const UNKNOWN_PROMPT: &str = "?";
const COMPOSE_MSG: &str = "Please enter your message and use /e to finish\n";

struct User {
    callsign: String,
    qth: String,
    total_session_bytes: usize
}

impl User {
    fn new() -> Self {
        Self {
            callsign: String::new(),
            qth: String::new(),
            total_session_bytes: 0,
        }
    }
}

fn screen_write(b: &str) {
    stdout().write_all(&b.as_bytes()).unwrap();
    stdout().flush().unwrap();
}

fn main() {
    let mut user: User = User::new();

    let mut in_buff = [0; MAX_BUFF];
    let mut out_buff = [0; MAX_BUFF];

    if let Ok(bc) = stdin().read(&mut in_buff[..MAX_CALL]){
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
        if let Ok(bc) = stdin().read(&mut in_buff[..1]){
            user.total_session_bytes += bc;
        }

        match in_buff[0] {
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

            // Send msg
            83 | 115 => {
                screen_write(COMPOSE_MSG);
                for (i, c) in stdin().bytes().enumerate() {
                    if let Ok(r) = c {
                        in_buff[i] = r
                    };

                    if i == MAX_BUFF {
                        return;
                    }

                    if i > 1 {
                        match in_buff.get(i - 1) {
                            Some(v) => {
                                if v == &47 && in_buff[i] == 101 {
                                    println!("Received: {} Bytes", i);
                                    return;
                                }
                            }
                            None => {}
                        }
                    }
                }
            }

            _ => {
                screen_write(UNKNOWN_PROMPT);
            }
        }
    }
}
