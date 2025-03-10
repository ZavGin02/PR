pub fn draw_rhombus(size: usize) {
    let mut rhombus = String::new();

    for i in 0..size {
        let spaces = size - i - 1;
        let stars = 2 * i + 1;
        rhombus.push_str(&" ".repeat(spaces));
        rhombus.push_str(&"*".repeat(stars));
        rhombus.push('\n');
    }

    for i in (0..size - 1).rev() {
        let spaces = size - i - 1;
        let stars = 2 * i + 1;
        rhombus.push_str(&" ".repeat(spaces));
        rhombus.push_str(&"*".repeat(stars));
        rhombus.push('\n');
    }

    print!("{}", rhombus);
}

fn main() {
    draw_rhombus(6); // Висота
}