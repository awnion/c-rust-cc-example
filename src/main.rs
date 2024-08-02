#[link(name = "sum", kind = "static")]
extern "C" {
    fn sum(a: *mut i32, b: *mut i32) -> i32;
}
fn main() {
    let mut a = 0;

    println!("Result {:?}", unsafe {
        sum(&mut a as *mut i32, &mut a as *mut i32)
    });
}
