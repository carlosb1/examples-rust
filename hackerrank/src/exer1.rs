static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn biggerIsGreater(w: &str) -> String {
    let splitted_str = w.to_string();
    let str_chars: std::str::Chars = splitted_str.chars();
    let tuples = indexes(str_chars.clone());
    let possible_values = sort(tuples.clone());
    println!("tuples: {:?}", tuples);
    println!("possible values: {:?}", possible_values);

    for letter in str_chars {
        println!("{:?}", letter);
    }

    "".to_string()
}

fn get_position(elems: Vec<(usize, char)>, letter: char) -> i32 {
    elems
        .iter()
        .enumerate()
        .find(|&v| {
            let value = *v.1;
            value.1 == letter
        })
        .map(|v| v.0 as i32)
        .unwrap_or(-1)
}

fn sum(elems: Vec<(usize, char)>) -> u64 {
    elems.iter().map(|(en, _)| *en as u64).sum()
}
fn sort(mut elems: Vec<(usize, char)>) -> Vec<(usize, char)> {
    elems.sort_by_key(|k| k.0);
    elems
}

fn indexes(chars: std::str::Chars) -> Vec<(usize, char)> {
    let elems: Vec<(usize, char)> = chars
        .into_iter()
        .map(|v| {
            let index = ASCII_LOWER.iter().position(|&r| r == v).unwrap();
            (index, v)
        })
        .collect();
    elems
}
pub fn main() {
    let inputs = vec!["ab", "bb", "hefg", "dhck", "dkhc"];
    for elem in inputs.clone() {
        let result = biggerIsGreater(elem);
    }
}
