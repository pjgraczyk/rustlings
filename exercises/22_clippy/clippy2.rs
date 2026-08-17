fn main() {
    let mut res = 42;
    let option = vec![Some(12), None, Some(40)];
    // TODO: Fix the Clippy lint.
    #[allow(for_loops_over_fallibles)]
    for x in option {
        for _y in x {
            let z: i32 = x.unwrap_or(0);
            res += z;
        }
    }

    println!("{res}");
}
