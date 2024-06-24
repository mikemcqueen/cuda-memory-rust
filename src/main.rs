#[link(name = "cuda_memory")]
extern "C" {
    fn get_cuda_memory(default_shared: *mut usize, max_shared: *mut usize, global: *mut usize) -> i32;
}

fn main() {
    let mut default_shared_memory: usize = 0;
    let mut max_shared_memory: usize = 0;
    let mut global_memory: usize = 0;

    unsafe {
        let result = get_cuda_memory(
            &mut default_shared_memory,
            &mut max_shared_memory,
            &mut global_memory
        );
        if result == 0 {
            println!("Default shared memory per block: {} bytes", default_shared_memory);
            println!("Maximum shared memory per block (with opt-in): {} bytes", max_shared_memory);
            println!("Total global memory: {} bytes", global_memory);
        } else {
            println!("Failed to get CUDA memory information");
        }
    }
}
