#![allow(unused)]

pub mod gen;
pub mod server;
pub mod terminal;
pub mod tools;
mod tests;

use petgraph::{adj::NodeIndex, Graph, Undirected};

use server::{Server, SkillArray};
use tools::Tools;

pub type Sector = Graph<Server, (), Undirected>;

pub struct State {
    pub sectors: Vec<Sector>,
    pub selected: (usize, NodeIndex),
    pub skills: SkillArray,
    pub tools: Tools
}
