use common::Solution;

pub struct Day04;

impl Day04 {
    fn find_overlaps<F>(f: F) -> usize
    where
        F: Fn(&u64, &u64, &u64, &u64) -> bool,
    {
        Self::input()
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
}

impl Solution for Day04 {
    fn input<'a>() -> &'a str {
        include_str!("../../input/day04.txt")
    }

    fn solve(&self) -> (usize, usize) {
        let part_one =
            Day04::find_overlaps(|l1, r1, l2, r2| (l2 <= l1 && r2 >= r1) || (l2 >= l1 && r2 <= r1));
        let part_two = Day04::find_overlaps(|l1, r1, l2, r2| !((l2 > r1) || (r2 < l1)));
        (part_one, part_two)
    }
}
