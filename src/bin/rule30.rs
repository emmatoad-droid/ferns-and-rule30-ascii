fn main() {
    const WIDTH: usize = 64;
    const STEPS: usize = 32;

    // Rule 30
    let rule: u8 = 30;
    let mut cells = vec![0u8; WIDTH];

    // Seed the initial state with a single active bit in the center
    cells[WIDTH / 2] = 1;

    println!("─── [ GENERATIVE CELLULAR AUTOMATON // RULE {} ] ───\n", rule);

    for _ in 0..STEPS {
        // Render current line to terminal
        let line: String = cells
            .iter()
            .map(|&bit| if bit == 1 { '█' } else { '·' })
            .collect();
        println!("{}", line);

        // Compute the next generation based on the 3-neighbourhood window
        let mut next_gen = vec![0u8; WIDTH];
        for i in 0..WIDTH {
            let left = if i == 0 { 0 } else { cells[i - 1] };
            let center = cells[i];
            let right = if i == WIDTH - 1 { 0 } else { cells[i + 1] };

            let neighbourhood = (left << 2) | (center << 1) | right;
            next_gen[i] = (rule >> neighbourhood) & 1;
        }
        cells = next_gen;
    }

    println!("\n─── [ EXECUTION COMPLETE // UNCLASSIFIED OUTPUT ] ───");
}
