use std::env;

use octree::Octree;
use vertex::VertexContainer;

mod octree;
mod octree_node;
mod svd;
mod vertex;

fn main() {
    println!("Starting...");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File path: {}", file_path);

    let pcl = VertexContainer::load_ply(file_path);

    let mut octree = Octree::new();

    octree.built_octree_from_vertex_container(&pcl, 0.2);

    println!("Point cloud points: {}", pcl.points.len());

    println!("Otree size: {}", octree.size);
    println!("Otree Total subdiv: {}", octree.total_subdiv);
}
