pub fn input() -> &'static str {
    include_str!("../../input/day04.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day04.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let part_one = find_overlaps(input, |l1, r1, l2, r2| {
        (l2 <= l1 && r2 >= r1) || (l2 >= l1 && r2 <= r1)
    });
    let part_two = find_overlaps(input, |l1, r1, l2, r2| !((l2 > r1) || (r2 < l1)));
    (part_one, part_two)
}

fn find_overlaps<F>(input: &str, f: F) -> usize
where
    F: Fn(&u64, &u64, &u64, &u64) -> bool,
{
    input
        .lines()
        .map(|pair| {
            pair.splitn(2, ',')
                .map(|range| {
                    range
                        .splitn(2, '-')
                        .map(|val| val.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|pair| {
            let [fst, snd] = &pair[..] else { unimplemented!() };
            let ([l1, r1], [l2, r2]) = (&fst[..], &snd[..]) else { unimplemented!() };
            assert!(r1 >= l1 && r2 >= l2, "{}-{},{}-{}", l1, r1, l2, r2);

            f(l1, r1, l2, r2)
        })
        .count()
}

common::test!(day04, (2, 4), (433, 852));
