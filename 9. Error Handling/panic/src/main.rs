fn main() {
    /*
        panic is unrecoverable error that crashes the program.
        use the `RUST_BACKTRACE` environment variable to show the backtrace
        ex: RUST_BACKTRACE=1 cargo run 
     */
    let v = vec![1, 2, 3];

    v[99];
    panic!("bum bum"); // manually panic the program
}
