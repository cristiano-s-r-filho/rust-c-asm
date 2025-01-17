use asm_cli::memory as env_memory;
fn main() {
    println!("Functioning as usual");     
    let init_tuples: (u16,u16,u16,u16) = env_memory::initiate_working_env(); 
    println!("INITIALIZATION COMPLETED! CODE -- {}:{}:{}:{}", init_tuples.0, init_tuples.1, init_tuples.2, init_tuples.3); 
}
