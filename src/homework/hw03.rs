const SIZE: usize = 17;

pub fn draw_square() {
    let mut square = String::new();

    for i in 0..SIZE {
        for j in 0..SIZE {
            if i == 0 || i == SIZE - 1 || j == 0 || j == SIZE - 1 {
                square.push('*');
            } else if i == j || j == SIZE - i - 1 {
                square.push('*');
            } else {
                square.push(' '); // Заповнюємо порожні місця
            }
        }
        square.push('\n');
    }

    print!("{}", square); // Один print!
}

fn main() {
    draw_square();
}