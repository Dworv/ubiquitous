use petgraph::adj::NodeIndex;
use rand::{thread_rng, Rng};

use crate::{Sector, server::{Server, FileSystem, ServerSecurity, SecurityState}};

pub fn gen_sector(level: f32) -> Sector {
    let base_req = if level == 1. { 1. } else { level * 5. };
    let mut sector = Sector::default();

    let starting_server = gen_server(base_req);
    
    // build nodes
    sector.add_node(starting_server);
    
    for layer in 1..=9 {
        let layer = layer as f32;
        let layer_incr = if level == 1. { 4./10. } else { 5./10. };
        let layer_req =  base_req + layer * layer_incr;
        for _ in 0..5 {
            sector.add_node(
                gen_server(layer_req)
            );
        }
    }

    sector.add_node(gen_server(level * 5.));

    // build edges
    for i in 1..=5 {
        sector.add_edge(0.into(), i.into(), ());
    }

    // connect intra-layer
    for layer in 1..=9 {
        for node in 0..5 {
            for _ in 0..thread_rng().gen_range(3..4) {
                let start = 1 + (layer - 1) * 5 + node;
                let end = 1 + (layer - 1) * 5 + thread_rng().gen_range(0..5);
                if start != end && !sector.contains_edge(start.into(), end.into()) {
                    sector.add_edge(start.into(), end.into(), ());
                }
            }
        }
    }

    // connect inter-layer
    for layer in 1..=8 {
        for node in 0..5 {
            for _ in 0..thread_rng().gen_range(3..4) {
                let start = 1 + (layer - 1) * 5 + node;
                let end_local = thread_rng().gen_range(0..5);
                let end = 1 + layer * 5 + end_local;
                if !sector.contains_edge(start.into(), end.into()) {
                    sector.add_edge(start.into(), end.into(), ());
                }
            }
        }
    }

    for i in 0..5 {
        sector.add_edge((1 + 8 * 5 + i).into(), (1 + 9 * 5).into(), ());
    }

    sector
}

fn gen_server(avg_skill: f32) -> Server {
    let skill_req = [0; 4].map(|_| gen_skill(avg_skill));
    let skill_req_root = [0; 4].map(|_| gen_skill(avg_skill + 0.5));

    Server {
        name: "bob".to_string(),
        fs: FileSystem::default(),
        sec: ServerSecurity {
            state: SecurityState::Secure,
            skill_req,
            skill_req_root
        }
    }
}

fn gen_skill(avg_skill: f32) -> f32 {
    thread_rng().gen_range((avg_skill-1.)..(avg_skill+1.))
}