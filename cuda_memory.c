#include <cuda_runtime.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

int get_cuda_memory(size_t* default_shared, size_t* max_shared, size_t* global) {
    struct cudaDeviceProp prop;
    cudaError_t error = cudaGetDeviceProperties(&prop, 0);
    
    if (error != cudaSuccess) {
        return 1;
    }
    
    *default_shared = prop.sharedMemPerBlock;
    *max_shared = prop.sharedMemPerBlockOptin;
    *global = prop.totalGlobalMem;
    
    return 0;
}

#ifdef __cplusplus
}
#endif
