// functions2.rs
// Make me compile! Execute `zustlings hint functions2` for hints :)


fn main() {
    call_this(3);
}

fn call_this(num:u64) {
    for i in 0..num {
        println!("Loop! number {}", i + 1);
    }
}
