mod test;

fn main() {}

pub fn romans(symbol: char) -> i16 {
    match symbol {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn roman_to_int(s: &str) -> i16 {
    let array = s.chars().map(|symbol| romans(symbol)).collect::<Vec<i16>>();

    /* Add the last element */
    let mut value = array[array.len() - 1];

    for i in 0..array.len() - 1 {
        let current = array[i];
        let next = array[i + 1];

        value += {
            if current < next {
                current * -1
            } else {
                current
            }
        };
    }
    value
}
