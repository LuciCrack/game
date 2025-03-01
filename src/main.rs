fn main() {
    // Create an instance for all backends
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::default(),
        backend_options: wgpu::BackendOptions::default(),
    });
    
    // Print info of adapters 
    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }
}
