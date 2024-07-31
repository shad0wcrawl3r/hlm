use hlm::cpu::get_cpu_info;

fn main() {
    let data = get_cpu_info();
    println!("{:?}", data);
}
