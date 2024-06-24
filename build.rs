use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=cuda_memory.c");

    // Compile the C code
    let output = Command::new("nvcc")
        .args(&["-c", "cuda_memory.c", "-o", "cuda_memory.o"])
        .output()
        .expect("Failed to compile CUDA C code");

    if !output.status.success() {
        panic!("CUDA compilation failed: {:?}", output);
    }

    // Create a static library
    let output = Command::new("ar")
        .args(&["crus", "libcuda_memory.a", "cuda_memory.o"])
        .output()
        .expect("Failed to create static library");

    if !output.status.success() {
        panic!("Failed to create static library: {:?}", output);
    }

    println!("cargo:rustc-link-search=native={}", std::env::current_dir().unwrap().display());
    println!("cargo:rustc-link-lib=static=cuda_memory");
    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=dylib=cudart");
}
