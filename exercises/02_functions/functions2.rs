// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: u8) {
    for i: u8 in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
