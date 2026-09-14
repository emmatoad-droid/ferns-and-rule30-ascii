fn main() {
    const SIZE: usize = 64;

    println!("\n        S I E R P I Ń S K I   T R I A N G L E\n");

    for y in 0..SIZE {
        // Centre the triangle.
        print!("{}", " ".repeat(SIZE - y));

        for x in 0..=y {
            if x & y == 0 {
                print!("██");
            } else {
                print!("  ");
            }
        }

        println!();
    }

    println!("\n        recursive geometry / bitwise structure\n");
}
