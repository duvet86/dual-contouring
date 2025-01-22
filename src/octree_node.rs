use crate::{octree::NodeParameters, svd::Vec3};

const CHILD_COUNT: usize = 8;

#[derive(Clone)]
pub struct OctreeNode {
    pub sub_nodes_has_object: bool,
    pub position: Vec3, // position of each node in xyz coords
    pub size: f32,
    pub node_idx: usize,
    pub sub_nodes: [Option<Box<OctreeNode>>; CHILD_COUNT],
    pub parent: Option<Box<OctreeNode>>,
    pub objects: Vec<Vec3>, // objects of this node. this can be points, vertices, etc.
}

impl OctreeNode {
    pub fn new_empty() -> Self {
        Self {
            sub_nodes_has_object: false,
            position: Vec3::new(0.0, 0.0, 0.0),
            size: 0.0,
            node_idx: 0,
            sub_nodes: Default::default(),
            parent: None,
            objects: Vec::new(),
        }
    }

    pub fn new(position: Vec3, size: f32, node_idx: usize) -> Self {
        Self {
            sub_nodes_has_object: false,
            position,
            size,
            node_idx,
            sub_nodes: Default::default(),
            parent: None,
            objects: Vec::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.sub_nodes[0].is_none()
    }

    pub fn subdivide_node(&mut self, nodes: &mut Vec<NodeParameters>) {
        // Mark the current node as not a leaf
        if let Some(ref mut node_param) = nodes.get_mut(self.node_idx) {
            node_param.is_leaf = false;
        }

        let mut new_pos;

        for i in 0..CHILD_COUNT {
            new_pos = self.position;
            /*LeftDownBack = 0,
            LeftUpBack = 2,
            RightDownBack = 4,
            RightUpBack = 6,

            LeftDownFront = 1,
            LeftUpFront = 3,
            RightDownFront = 5,
            RightUpFront = 7,*/
            /*
                We have _ _ _ = X Y Z, so 100 = 4 ->RIGHT
                                          010 = 2 -> UP
                                          001 = 1 -> FRONT
            */
            if i == 4 || i == 6 || i == 5 || i == 7 {
                //right
                new_pos.x += self.size * 0.25;
            } else {
                new_pos.x -= self.size * 0.25;
            }

            if i == 2 || i == 6 || i == 3 || i == 7 {
                //up
                new_pos.y += self.size * 0.25;
            } else {
                new_pos.y -= self.size * 0.25;
            }

            if i == 1 || i == 3 || i == 5 || i == 7 {
                //front
                new_pos.z += self.size * 0.25;
            } else {
                new_pos.z -= self.size * 0.25;
            }

            self.sub_nodes[i] = Some(Box::new(OctreeNode {
                sub_nodes_has_object: false,
                position: new_pos,
                size: self.size * 0.5,
                node_idx: nodes.len(),
                sub_nodes: Default::default(),
                parent: Some(Box::new(self.clone())),
                objects: Vec::new(),
            }));

            nodes.push(NodeParameters::new(
                self.sub_nodes[i].as_ref().unwrap().position,
                self.size * 0.5,
            ));
        }
    }

    pub fn get_index_by_position(&self, look_up_position: &Vec3) -> Option<usize> {
        if self.is_leaf() {
            return None;
        }

        let mut index = 0;
        if look_up_position.x > self.position.x {
            index += 4;
        }
        if look_up_position.y > self.position.y {
            index += 2;
        }
        if look_up_position.z > self.position.z {
            index += 1;
        }

        Some(index)
    }

    pub fn mark_parent(&mut self) {
        let mut current = self;

        while let Some(ref mut parent) = current.parent {
            current.sub_nodes_has_object = true;
            current = parent;
        }
    }
}
