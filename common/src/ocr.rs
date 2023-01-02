use std::collections::HashMap;

fn characters() -> HashMap<&'static str, char> {
    HashMap::from([
        (".##.\n#..#\n#..#\n####\n#..#\n#..#", 'A'),
        ("###.\n#..#\n###.\n#..#\n#..#\n###.", 'B'),
        (".##.\n#..#\n#...\n#...\n#..#\n.##.", 'C'),
        ("####\n#...\n###.\n#...\n#...\n####", 'E'),
        ("####\n#...\n###.\n#...\n#...\n#...", 'F'),
        (".##.\n#..#\n#...\n#.##\n#..#\n.###", 'G'),
        ("#..#\n#..#\n####\n#..#\n#..#\n#..#", 'H'),
        (".###\n..#.\n..#.\n..#.\n..#.\n.###", 'I'),
        ("..##\n...#\n...#\n...#\n#..#\n.##.", 'J'),
        ("#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#", 'K'),
        ("#...\n#...\n#...\n#...\n#...\n####", 'L'),
        (".##.\n#..#\n#..#\n#..#\n#..#\n.##.", 'O'),
        ("###.\n#..#\n#..#\n###.\n#...\n#...", 'P'),
        ("###.\n#..#\n#..#\n###.\n#.#.\n#..#", 'R'),
        (".###\n#...\n#...\n.##.\n...#\n###.", 'S'),
        ("#..#\n#..#\n#..#\n#..#\n#..#\n.##.", 'U'),
        ("#...\n#...\n.#.#\n..#.\n..#.\n..#.", 'Y'),
        ("####\n...#\n..#.\n.#..\n#...\n####", 'Z'),
    ])
}

pub fn convert_letter(input: &str, idx: usize) -> Option<char> {
    let offset = idx * 4 + idx;
    let c = input
        .lines()
        .map(|line| {
            line.chars()
                .skip(offset)
                .take(4)
                .fold(String::new(), |acc, s| acc + &s.to_string())
        })
        .fold(String::new(), |acc, s| acc + &s + "\n");

    characters().get(&c.trim()).cloned()
}

pub fn convert(input: &str) -> String {
    (0..)
        .take_while(|&idx| convert_letter(input, idx).is_some())
        .map(|idx| convert_letter(input, idx).unwrap())
        .fold(String::new(), |acc, s| acc + &s.to_string())
}
