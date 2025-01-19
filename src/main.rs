mod qef;
mod svd;

fn main() {
    println!("Hello, world!");
}

type VertexBuffer = Vec<MeshVertex>;
type IndexBuffer = Vec<u64>;

struct MeshVertex {
    xyz: svd::Vec3,
    normal: svd::Vec3,
}

// impl MeshVertex {
//     pub fn new(v: glm::Vec3, n: &glm::Vec3) -> MeshVertex {
//         MeshVertex {
//             xyz: glm::Vec3::new(v.y, v.x, v.z),
//             normal: glm::normalize(n),
//         }
//     }
// }

// enum OctreeNodeType {
//     NodeNone,
//     NodeInternal,
//     NodePsuedo,
//     NodeLeaf,
// }

// struct OctreeDrawInfo {
//     index: u32,
//     corners: u32,
//     position: glm::Vec3,
//     average_normal: glm::Vec3,
//     qef: qef::QefData,
// }
