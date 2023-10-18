use petgraph::graph::NodeIndex;

use super::{AttackInfo, SecurityState, Server};
use crate::Sector;

pub fn hack(
    sector: &mut Sector,
    index: NodeIndex,
    attack: AttackInfo,
    name: String,
) -> Option<bool> {
    let mut target_index = None;
    for neighbor_index in sector.neighbors(index) {
        if sector.node_weight(neighbor_index).unwrap().name == name {
            target_index = Some(neighbor_index);
            break;
        }
    }
    target_index.map(|target| {
        let security = &mut sector.node_weight_mut(target).unwrap().sec;
        if security.can_compromise(&attack) {
            security.state = SecurityState::Compromised;
            true
        } else {
            false
        }
    })
}

pub fn ls(sector: &Sector, index: NodeIndex) -> Vec<String> {
    let server = sector.node_weight(index).unwrap();
    let fs = &server.fs;
    let mut names = vec![];
    for file in &fs.files {
        names.push(file.name.clone());
    }
    names
}

pub fn lsdev(sector: &Sector, index: NodeIndex) -> Vec<String> {
    let server = sector.node_weight(index).unwrap();
    let devices = &server.devices;
    let mut names = vec![];
    for dev in devices {
        names.push(dev.lsname().to_string());
    }
    names
}

pub fn lsnet(sector: &Sector, index: NodeIndex) -> Vec<(String, SecurityState)> {
    let mut infos = vec![];
    for node in sector.neighbors(index) {
        let server = sector.node_weight(node).unwrap();
        infos.push((server.name.clone(), server.sec.state.clone()));
    }
    infos
}

pub enum CatErr {
    NotFound,
    Unreadable
}

pub fn cat(sector: &Sector, index: NodeIndex, name: &str) -> Result<String, CatErr> {
    let server = sector.node_weight(index).unwrap();
    let fs = &server.fs;
    for file in &fs.files {
        if file.name == name {
            match file.content.get() {
                Some(content) => Ok(content),
                None => Err(CatErr::Unreadable),
            };
        }
    }
    Err(CatErr::NotFound)
}
