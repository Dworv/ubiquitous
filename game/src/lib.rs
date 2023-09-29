#![allow(unused)]

pub mod gen;
pub mod server;
pub mod terminal;

mod tests;

use petgraph::{Graph, Undirected, adj::NodeIndex};
use server::Server;

pub type Sector = Graph<Server, (), Undirected>;

pub struct State {
    pub sectors: Vec<Sector>,
    pub selected: (usize, NodeIndex),
}
