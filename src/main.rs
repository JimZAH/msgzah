use std::io::{stdin, stdout, Read, Write};
use std::str;

// Buffer sizes
const MAX_CALL: usize = 8;
const MAX_BUFF: usize = 4 * 1024;

// Messages
const COMMANDS_MSG: &str = "H: Help, S: Compose Message, L: List Messages, E: Exit\n";
const WELCOME_MSG: &str = "You have connected to M0ZAH Mailbox\nMSGZAH Version: 0.1";
const USER_PROMPT: &str = ">>>";
const COMPOSE_MSG: &str = "Please enter your message and use /e to finish\n";

struct User {
    callsign: String,
    qth: String,
}

impl User {
    fn new() -> Self {
        Self {
            callsign: String::new(),
            qth: String::new(),
        }
    }
}

fn main() {
    let mut user: User = User::new();

    let mut in_buff = [0; MAX_BUFF];
    let mut out_buff = [0; MAX_BUFF];

    stdin().read(&mut in_buff).unwrap();

    user.callsign = match str::from_utf8(&in_buff[..MAX_CALL]) {
        Ok(v) => v.to_owned(),
        Err(_) => "N0CALL".to_owned(),
    };

    let welcome = format!(
        "Welcome: {}{}\n\n{}\n",
        user.callsign, WELCOME_MSG, COMMANDS_MSG
    );

    stdout().write_all(welcome.as_bytes()).unwrap();

    loop {
        stdin().read(&mut in_buff).unwrap();

        match in_buff[0] {
            // Exit
            69 | 101 => return,

            // Help
            72 | 104 => {
                stdout().write_all(COMMANDS_MSG.as_bytes()).unwrap();
                stdout().flush().unwrap();
            }

            // List Messages
            76 | 108 => {}

            // Send msg
            83 | 115 => {
                stdout().write_all(COMPOSE_MSG.as_bytes()).unwrap();
                stdout().flush().unwrap();
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
                                    // For testing print the message back
                                    let msg = match str::from_utf8(&in_buff[..i - 1]) {
                                        Ok(v) => v.to_owned(),
                                        Err(_) => "EMPTY".to_owned(),
                                    };
                                    println!("{}", msg);
                                    return;
                                }
                            }
                            None => {}
                        }
                    }
                }
            }

            _ => {
                stdout().write_all(USER_PROMPT.as_bytes()).unwrap();
                stdout().flush().unwrap();
            }
        }
    }
}
