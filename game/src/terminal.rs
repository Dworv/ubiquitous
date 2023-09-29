use std::io::stdin;

use crate::{gen::gen_sector, server::{Server, File}, State};

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
        for line in stdin().lines().map(|x| x.unwrap()) {
            let mut words = line.split(' ');
            if let Some(command) = words.next() {
                match command {
                    "ls" => {
                        let (area, server) = self.state.selected;
                        let fs = &self.state.sectors[area].node_weight(server.into()).unwrap().fs;
                        for file in &fs.files {
                            println!("{}", file.name);
                        }
                    }
                    "lsdev" => {
                        let (area, server) = self.state.selected;
                        let devices = &self.state.sectors[area].node_weight(server.into()).unwrap().devices;
                        for device in devices {
                            println!("{}", device.lsname());
                        }
                    }
                    _ => {
                        println!("command not found")
                    }
                }
            }
        }
    }
}
