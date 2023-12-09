fn main() {
    let input = std::io::stdin().lines().flatten().collect::<Vec<_>>();
    let mut p = input
        .iter()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .map(|(hands, bid)| (hands.chars().map(Cards::from).collect::<Vec<_>>(), bid))
        .map(|(hands, bid)| {
            let all_possible_elements = hands.iter().fold(Vec::new(), |mut acc, x| {
                if !acc.contains(x) {
                    acc.push(*x);
                }
                acc
            });
            let max_without_j = all_possible_elements
                .iter()
                .filter(|&&x| x != Cards::J)
                .map(|element| (element, hands.iter().filter(|x| x == &element).count()))
                .max_by_key(|x| x.1)
                .map(|x| x.0);

            let mut type_matches = all_possible_elements
                .iter()
                .map(|elem| {
                    if Some(elem) == max_without_j {
                        hands
                            .iter()
                            .filter(|&x| x == elem || x == &Cards::J)
                            .count()
                    } else {
                        if max_without_j == None {
                            5
                        } else if elem != &Cards::J {
                            hands.iter().filter(|&x| x == elem).count()
                        } else {
                            1
                        }
                    }
                })
                .collect::<Vec<_>>();
            type_matches.sort();
            let type_matches = type_matches.into_iter().rev().collect::<Vec<_>>();
            let a = match type_matches {
                x if x[0] == 5 => MatchKind::ExactIdentical,
                x if x[0] == 4 => MatchKind::FourIdentical,
                x if x[0] == 3 => {
                    let p = if let Some(x) = x.get(1) {
                        *x == 2
                    } else {
                        false
                    };
                    if p {
                        MatchKind::FullHouse
                    } else {
                        MatchKind::ThreeIdentical
                    }
                }
                x if x[0] == 2 => {
                    let p = if let Some(x) = x.get(1) {
                        *x == 2
                    } else {
                        false
                    };
                    if p {
                        MatchKind::TwoPair
                    } else {
                        MatchKind::OnePair
                    }
                }
                x if x[0] == 1 => MatchKind::AllDistinct,
                _ => unreachable!(),
            };
            ((hands, a), bid)
        })
        .collect::<Vec<_>>();
    p.sort_by_key(|(x, _)| x.1);
    let all_possible_pairs = p.iter().fold(Vec::new(), |mut acc, x| {
        let kind = x.0 .1;
        if !acc.contains(&kind) {
            acc.push(kind);
        }
        acc
    });
    let t = all_possible_pairs.iter().fold(Vec::new(), |mut acc, x| {
        let a = p
            .iter()
            .filter(|((_, match_k), _)| match_k == x)
            .collect::<Vec<_>>();
        acc.push(a);
        acc
    });
    let ans = t
        .into_iter()
        .rev()
        .flat_map(|mut kind| {
            kind.sort_by(|((a, _), _), ((b, _), _)| {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(b).unwrap() {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => {}
                    }
                }
                unreachable!();
            });
            kind.into_iter()
        })
        .collect::<Vec<_>>();
    dbg!(&ans
        .iter()
        .filter(|x| x.0 .0.contains(&Cards::J)
            && x.0 .1 != MatchKind::ExactIdentical
            && x.0 .1 != MatchKind::FourIdentical
            && x.0 .1 != MatchKind::FullHouse
            // && x.0 .1 != MatchKind::ThreeIdentical
            && x.0 .1 != MatchKind::OnePair)
        .collect::<Vec<_>>());
    // dbg!(&ans);
    let ans = ans.iter().enumerate().fold(0, |acc, (rank, x)| {
        let bid = x.1.parse::<usize>().unwrap();
        let total = (rank + 1) * bid;
        acc + total
    });
    dbg!(&ans);
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, PartialOrd, Ord)]
enum MatchKind {
    ExactIdentical,
    FourIdentical,
    FullHouse,      // three same, the other two also same
    ThreeIdentical, // three same, the other two not same,
    TwoPair,        // two same, the other two same, last not same
    OnePair,        // two same, the other three not same
    AllDistinct,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Cards {
    J,
    Numbers(u32),
    T,
    Q,
    K,
    A,
}

impl From<char> for Cards {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            x if x.is_digit(10) => Self::Numbers(x.to_digit(10).unwrap()),
            _ => panic!("t"),
        }
    }
}
