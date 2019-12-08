use lazy_static::lazy_static;

type Image = Vec<[[u8; 25]; 6]>;

lazy_static! {
    static ref IMAGE: Image = include_str!("input.txt")
        .trim()
        .as_bytes()
        .chunks(25 * 6)
        .map(|data| {
            let mut layer = [[0; 25]; 6];
            for (i, chunk) in data.chunks(25).enumerate() {
                layer[i].copy_from_slice(chunk);
            }
            layer
        })
        .collect();
}

fn part1() {
    let layer = IMAGE
        .iter()
        .min_by_key(|layer| {
            layer
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&p| p == b'0')
                .count()
        })
        .unwrap();
    let mut ones = 0;
    let mut twos = 0;
    for p in layer.iter().flat_map(|row| row.iter()) {
        match p {
            b'1' => ones += 1,
            b'2' => twos += 1,
            _ => {}
        };
    }
    println!("{}", ones * twos);
}

fn part2() {
    let mut layers = IMAGE.iter();
    let mut image = layers.next().unwrap().clone();
    for layer in layers {
        for i in 0..6 {
            for j in 0..25 {
                if image[i][j] == b'2' {
                    image[i][j] = layer[i][j];
                }
            }
        }
    }
    for i in 0..6 {
        for j in 0..25 {
            match image[i][j] {
                b'0' => print!("  "),
                b'1' => print!("██"),
                _ => {}
            }
        }
        println!();
    }
}

fn main() {
    part1();
    part2();
}
