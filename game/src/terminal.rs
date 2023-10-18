use std::io::{stdin, stdout, Write};

use crate::{
    gen::gen_sector,
    server::{utils, AttackInfo, AttackKind, File, Server, FileContent},
    State, tools::Tools,
};

pub struct Terminal {
    state: State,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            state: State {
                sectors: vec![gen_sector(1.)],
                selected: (0, 0),
                skills: [1.; 4],
                tools: Tools::default()
            },
        }
    }

    pub fn run(&mut self) {
        let fs = &mut self.state.sectors[0].node_weight_mut(0.into()).unwrap().fs;
        fs.files.push(File { name: "bazinga".to_string(), content: FileContent::Text("oh".to_string()) });
        print!(
            "user@{}:~$ ",
            self.state.sectors[0].node_weight(0.into()).unwrap().name
        );
        stdout().flush().unwrap();
        for line in stdin().lines().map(|x| x.unwrap()) {
            let mut words = line.split(' ');
            let (sector, server) = self.state.selected;
            let sector = &mut self.state.sectors[sector];
            if let Some(command) = words.next() {
                match command {
                    "hack" => {
                        if let Some(kind) = words.next() {
                            if let Some(name) = words.next() {
                                let attack = match kind {
                                    "pwd" | "password" | "crack" => Some(AttackInfo {
                                        kind: AttackKind::Password,
                                        skill: self.state.skills[0],
                                    }),
                                    "proto" | "protomanip" => Some(AttackInfo {
                                        kind: AttackKind::ProtoManip,
                                        skill: self.state.skills[1],
                                    }),
                                    "impersonation" | "phish" | "phishing" => Some(AttackInfo {
                                        kind: AttackKind::Impersonation,
                                        skill: self.state.skills[2],
                                    }),
                                    "collision" | "hash" | "hashing" => Some(AttackInfo {
                                        kind: AttackKind::Impersonation,
                                        skill: self.state.skills[3],
                                    }),
                                    _ => {
                                        dbg!();
                                        None
                                    }
                                };
                                if let Some(attack) = attack {
                                    match utils::hack(
                                        sector,
                                        server.into(),
                                        attack,
                                        name.to_string(),
                                    ) {
                                        Some(true) => println!("Hacking was a success!"),
                                        Some(false) => println!("Hacking failed..."),
                                        None => println!("ERROR: Server could not be not found"),
                                    }
                                } else {
                                    println!("accepts kinds of attacks are password, ");
                                }
                            }
                        } else {
                            println!("usage: hack <kind> <server_name>");
                        }
                    }
                    "cat" => {
                        if let Some(name) = words.next() {
                            match utils::cat(sector, server.into(), name) {
                                Ok(content) => println!("{}", content),
                                Err(utils::CatErr::Unreadable) => println!("File content cannot be read"),
                                Err(utils::CatErr::NotFound) => println!("File not found")
                            }
                        } else {
                            println!("usage: cat <file>");
                        }
                    }
                    "ls" => {
                        for name in utils::ls(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    "lsdev" => {
                        for name in utils::lsdev(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    "lsnet" => {
                        for (name, state) in utils::lsnet(sector, server.into()) {
                            let state = match state {
                                crate::server::SecurityState::Secure => "secure",
                                crate::server::SecurityState::Compromised => "compromised",
                                crate::server::SecurityState::CompromisedRoot => "compromised (root)",
                            };
                            println!("{:<12} {}", name, state);
                        }
                    }
                    _ => {
                        println!("Command not found.")
                    }
                }
            }
            print!(
                "user@{}:~$ ",
                sector.node_weight(server.into()).unwrap().name
            );
            stdout().flush().unwrap();
        }
    }
}
