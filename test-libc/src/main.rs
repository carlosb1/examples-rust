use libc::pid_t;

#[link(name = "c")]
extern "C" {
    fn getpid() -> pid_t;
}

fn main() {
    let x = unsafe { getpid() };
    println!("Process PID {}", x);
}
