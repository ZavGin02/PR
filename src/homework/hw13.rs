#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rectangle {
    pub a: Point,
    pub b: Point,
}

pub fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut grid = [[false; 100]; 100];
    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                grid[x as usize][y as usize] = true;
            }
        }
    }
    let mut count = 0;
    for row in grid.iter() {
        for &cell in row.iter() {
            if cell {
                count += 1;
            }
        }
    }
    count
}

pub fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}