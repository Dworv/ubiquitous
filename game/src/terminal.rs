use std::io::{stdin, stdout, Write};

use crate::{
    gen::gen_sector,
    server::{utils, File, Server},
    State,
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
            },
        }
    }

    pub fn run(&mut self) {
        let fs = &mut self.state.sectors[0].node_weight_mut(0.into()).unwrap().fs;
        fs.files.push(File::new("bazinga".to_string()));
        print!("user@{}:~$ ", self.state.sectors[0].node_weight(0.into()).unwrap().name);
        stdout().flush().unwrap();
        for line in stdin().lines().map(|x| x.unwrap()) {
            let mut words = line.split(' ');
            let (sector, server) = self.state.selected;
            let sector = &mut self.state.sectors[sector];
            if let Some(command) = words.next() {
                match command {
                    "cat" => {
                        if let Some(name) = words.next() {
                            if let Some(contents) = utils::cat(sector, server.into(), name) {
                                println!("{}", contents);
                            } else {
                                println!("file not found");
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
                        for name in utils::lsnet(sector, server.into()) {
                            println!("{}", name);
                        }
                    }
                    _ => {
                        println!("command not found")
                    }
                }
            }
            print!("user@{}:~$ ", sector.node_weight(server.into()).unwrap().name);
            stdout().flush().unwrap();
        }
    }
}
