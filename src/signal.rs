#[derive(Clone, Copy)]
pub struct Node {
    pub dependents: [i32; 64],
    pub dep_count: i32,
}

pub struct Graph {
    pub nodes: [Node; 4096],
    pub node_count: i32,
    pub update_buffer: [i32; 4096],
    pub update_count: i32,
}

#[no_mangle]
pub extern "C" fn init_graph() -> *mut Graph {
    let g = Box::new(Graph {
        nodes: [Node { dependents: [0; 64], dep_count: 0 }; 4096],
        node_count: 0,
        update_buffer: [0; 4096],
        update_count: 0,
    });
    Box::into_raw(g)
}
