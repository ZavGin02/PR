fn draw_tree(triangle_count: usize) {
    let base_height = 6;
    let max_width = 2 * usize::max(triangle_count, base_height) - 1;

    (0..triangle_count).for_each(|t| {
        let base_width = 2 * (t + 1) - 1;

        (0..=t).for_each(|row| {
            let stars = 2 * row + 1;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        });
    });

    (0..base_height).for_each(|row| {
        let stars = 2 * row + 1;
        let spaces = (max_width - stars) / 2;
        println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
    });
}

fn main() {
    let triangle_count = 5;
    draw_tree(triangle_count);
}
