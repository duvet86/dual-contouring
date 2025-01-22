use std::f32::INFINITY;

use crate::{octree_node::OctreeNode, svd::Vec3, vertex::VertexContainer};

pub struct NodeParameters {
    pub coords: Vec3,
    pub size: f32,
    pub is_leaf: bool,
    pub has_object: bool,
}

impl NodeParameters {
    pub fn new(coords: Vec3, size: f32) -> Self {
        Self {
            coords,
            size,
            is_leaf: true,
            has_object: false,
        }
    }
}

pub struct Octree {
    pub root: OctreeNode,
    pub nodes: Vec<NodeParameters>,
    pub total_subdiv: usize,
    pub size: f32, // OK
    pub threshold: f32,
}

impl Octree {
    pub fn new() -> Self {
        Self {
            root: OctreeNode::new_empty(),
            nodes: Vec::new(),
            total_subdiv: 0,
            size: -1.0,
            threshold: INFINITY,
        }
    }

    pub fn built_octree_from_vertex_container(
        &mut self,
        pcl: &VertexContainer,
        closeness_threshold: f32,
    ) {
        self.size = Self::get_size_from_vertex_container(pcl);
        self.threshold = closeness_threshold;
        self.root.position = pcl.geometric_center_point;
        self.root.size = self.size;
        self.root.node_idx = self.nodes.len();

        let node_parameters = NodeParameters::new(self.root.position, self.size);
        self.nodes.push(node_parameters);

        let mut current_node = &mut self.root;

        for point in pcl.points.iter() {
            if Self::sqr_distance(current_node.position, point.coords) > closeness_threshold {
                self.subdivision(&point.coords);
                current_node = &mut self.root; // Go back to the root for the next point
            }
        }
    }

    fn get_size_from_vertex_container(pcl: &VertexContainer) -> f32 {
        let mut temp_size = pcl.rigth_boundary_point.coords.x - pcl.left_boundary_point.coords.x;

        if (pcl.up_boundary_point.coords.y - pcl.down_boundary_point.coords.y) > temp_size {
            temp_size = pcl.up_boundary_point.coords.y - pcl.down_boundary_point.coords.y;
        }
        if (pcl.front_boundary_point.coords.z - pcl.back_boundary_point.coords.z) > temp_size {
            temp_size = pcl.front_boundary_point.coords.z - pcl.back_boundary_point.coords.z;
        }

        temp_size
    }

    fn sqr_distance(point1: Vec3, point2: Vec3) -> f32 {
        (point1.x - point2.x).powi(2)
            + (point1.y - point2.y).powi(2)
            + (point1.z - point2.z).powi(2)
    }

    fn subdivision(&mut self, point: &Vec3) {
        let mut current_node = &mut self.root;

        while current_node.size > self.threshold {
            if current_node.is_leaf() {
                current_node.subdivide_node(&mut self.nodes);
                self.total_subdiv += 1;
            }
            let index = current_node.get_index_by_position(point).unwrap();
            if let Some(ref mut sub_node) = current_node.sub_nodes[index] {
                current_node = sub_node;
            } else {
                break;
            }
        }
        current_node.mark_parent();
        self.nodes[current_node.node_idx].has_object = true;
        current_node.objects.push(*point);
    }
}
