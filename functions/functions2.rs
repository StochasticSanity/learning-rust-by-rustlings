// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring-a-Ding-Ding! Call number {}", i + 1);
    }
}
