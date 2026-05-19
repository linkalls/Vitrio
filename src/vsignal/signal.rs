#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

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

static mut GRAPH: Graph = Graph {
    nodes: [Node { dependents: [0; 64], dep_count: 0 }; 4096],
    node_count: 0,
    update_buffer: [0; 4096],
    update_count: 0,
};

static mut VISITED_WORDS: [i32; 128] = [0; 128];
static mut QUEUE: [i32; 4096] = [0; 4096];

#[no_mangle]
pub extern "C" fn init_graph() -> *mut Graph {
    unsafe {
        GRAPH.node_count = 0;
        GRAPH.update_count = 0;
        &raw mut GRAPH
    }
}

#[no_mangle]
pub extern "C" fn create_node(g_ptr: *mut Graph) -> i32 {
    unsafe {
        let g = &mut *g_ptr;
        let id = g.node_count;
        g.node_count += 1;
        id
    }
}

#[no_mangle]
pub extern "C" fn add_dependency(g_ptr: *mut Graph, dependent: i32, dependency: i32) {
    unsafe {
        let g = &mut *g_ptr;
        let cnt = g.nodes[dependency as usize].dep_count;

        if cnt > 0 && g.nodes[dependency as usize].dependents[(cnt - 1) as usize] == dependent {
            return;
        }

        g.nodes[dependency as usize].dependents[cnt as usize] = dependent;
        g.nodes[dependency as usize].dep_count += 1;
    }
}

#[no_mangle]
pub extern "C" fn propagate(g_ptr: *mut Graph, source_id: i32) -> i32 {
    unsafe {
        let g = &mut *g_ptr;
        g.update_count = 0;

        let mut q_head = 0;
        let mut q_tail = 0;

        let cnt = g.nodes[source_id as usize].dep_count;
        for i in 0..cnt {
            let dep = g.nodes[source_id as usize].dependents[i as usize];
            QUEUE[q_tail as usize] = dep;
            q_tail += 1;
            VISITED_WORDS[(dep >> 5) as usize] |= 1 << (dep & 31);
        }

        while q_head < q_tail {
            let curr = QUEUE[q_head as usize];
            q_head += 1;

            g.update_buffer[g.update_count as usize] = curr;
            g.update_count += 1;

            let d_cnt = g.nodes[curr as usize].dep_count;
            for i in 0..d_cnt {
                let dep = g.nodes[curr as usize].dependents[i as usize];
                let word_idx = (dep >> 5) as usize;
                let mask = 1 << (dep & 31);

                if (VISITED_WORDS[word_idx] & mask) == 0 {
                    VISITED_WORDS[word_idx] |= mask;
                    QUEUE[q_tail as usize] = dep;
                    q_tail += 1;
                }
            }
        }

        // Clear the visited bits for the next call to avoid memset overhead
        for i in 0..q_tail {
            let dep = QUEUE[i as usize];
            VISITED_WORDS[(dep >> 5) as usize] = 0;
        }

        g.update_count
    }
}

#[no_mangle]
pub extern "C" fn get_update_buffer_ptr(g_ptr: *mut Graph) -> *mut i32 {
    unsafe {
        let g = &mut *g_ptr;
        g.update_buffer.as_mut_ptr()
    }
}
