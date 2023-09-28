#![allow(unused)]

pub mod gen;
pub mod server;
pub mod terminal;

mod tests;

use petgraph::{Graph, Undirected};
use server::Server;

pub type Sector = Graph<Server, (), Undirected>;

pub struct State {
    // will be tree
    pub sectors: Vec<Sector>,
    pub selected: usize,
}
