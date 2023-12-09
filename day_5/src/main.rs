use std::{collections::HashMap, thread, time::Duration};

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build_global()
        .unwrap();
    let mut input = std::io::stdin().lines().flatten().collect::<Vec<_>>();
    let seeds = input[0]
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter_map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let seeds = seeds
        .chunks(2)
        .map(|x| (x[0]..).take(x[1] as usize).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    input.extend(["hu-tt-to map:", "33 33 33", "33 24 43"].map(str::to_string));
    let p = input
        .iter()
        .enumerate()
        .filter_map(|x| x.1.contains(':').then_some((x.0, x.1)))
        .collect::<Vec<_>>();
    let p = &p[1..];
    let map = p.windows(2).fold(Vec::new(), |mut acc, x| {
        let (u1, x1) = x[0];
        let x1 = x1.split('-').collect::<Vec<_>>();
        let (source, target) = (x1[0], x1[2].split(' ').next().unwrap());
        let (u2, _) = x[1];
        let a = &input[u1 + 1..u2 - 1];
        let p = a
            .iter()
            .map(|x| {
                x.split(' ')
                    .map(str::trim)
                    .filter_map(|x| x.parse::<i64>().ok())
                    .collect::<Vec<_>>()
            })
            .map(|x| (x[1], x[0], x[2])) // source, dest, len
            .collect::<Vec<_>>();
        acc.push((source, p));
        acc
    });
    dbg!(&map);
    dbg!("I'm fucked");
    thread::sleep(Duration::from_millis(1000));
    let mut p = seeds
        .into_par_iter()
        .fold(
            || Vec::new(),
            |mut acc, p| {
                let a = map.iter().fold(p, |mut acc, x| {
                    for (source, dest, len) in &x.1 {
                        let dest_upper = dest + len;

                        let diff = acc - source;
                        if diff >= 0 {
                            let desired_value = dest + diff;
                            if source < dest {
                                if desired_value < *source {
                                    continue;
                                }
                            } else {
                                if desired_value < *dest {
                                    continue;
                                }
                            }
                            if desired_value <= dest_upper {
                                acc = desired_value;
                                break;
                            }
                        }
                    }
                    acc
                });
                acc.push((p, a));
                acc
            },
        )
        .reduce(
            || Vec::new(),
            |mut a, b| {
                a.extend(b);
                a
            },
        );
    let p = p.iter().min_by_key(|x| x.1).unwrap();
    dbg!(p);

    // dbg!(map);
}
