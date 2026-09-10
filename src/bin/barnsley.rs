use std::collections::HashSet;

fn main() {
    const WIDTH: usize = 65;
    const HEIGHT: usize = 32;
    const ITERATIONS: usize = 40_000;

    // Grid buffer using a HashSet to store unique coordinate points
    let mut grid = HashSet::new();

    // Initial state vector
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    // Simple pseudo-random linear congruential generator for deterministic sampling
    let mut lcg_state: u64 = 0xDEADBEEF;

    println!("─── [ ITERATED FUNCTION SYSTEM // BARNSLEY FERN ] ───\n");

    for _ in 0..ITERATIONS {
        // Generate a value between 0 and 99
        lcg_state = lcg_state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let prob = (lcg_state % 100) as u8;

        // Apply the 4 affine transformations based on probability thresholds
        let (next_x, next_y) = match prob {
            0 => (0.0, 0.16 * y), // Stem (1%)
            1..=85 => (
                0.85 * x + 0.04 * y,
                -0.04 * x + 0.85 * y + 1.6,
            ), // Successively smaller leaflets (85%)
            86..=92 => (
                0.20 * x - 0.26 * y,
                0.23 * x + 0.22 * y + 1.6,
            ), // Largest left leaflet (7%)
            _ => (
                -0.15 * x + 0.28 * y,
                0.26 * x + 0.24 * y + 0.44,
            ), // Largest right leaflet (7%)
        };

        x = next_x;
        y = next_y;

        // Map continuous fractal coordinates into character grid space
        // Known ranges: x approx [-2.182, 2.6558], y approx [0, 9.9983]
        let grid_x = (((x + 2.5) / 5.0) * (WIDTH as f64 - 1.0)).floor() as isize;
        let grid_y = (((10.0 - y) / 10.0) * (HEIGHT as f64 - 1.0)).floor() as isize;

        if (0..(WIDTH as isize)).contains(&grid_x) && (0..(HEIGHT as isize)).contains(&grid_y) {
            grid.insert((grid_x as usize, grid_y as usize));
        }
    }

    // Render the grid buffer to stdout
    for row in 0..HEIGHT {
        let mut line = String::with_capacity(WIDTH);
        for col in 0..WIDTH {
            if grid.contains(&(col, row)) {
                line.push('█');
            } else {
                line.push('·');
            }
        }
        println!("{}", line);
    }

    println!("\n─── [ EXECUTION COMPLETE // UNCLASSIFIED OUTPUT ] ───");
}
