#![allow(unused)]

pub mod gen;
pub mod server;
pub mod terminal;

mod tests;

use petgraph::{adj::NodeIndex, Graph, Undirected};
use server::Server;

pub type Sector = Graph<Server, (), Undirected>;

pub struct State {
    pub sectors: Vec<Sector>,
    pub selected: (usize, NodeIndex),
}
