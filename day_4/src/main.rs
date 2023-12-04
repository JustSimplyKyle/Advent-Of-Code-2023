use std::collections::HashMap;

fn main() {
    let input = std::io::stdin().lines().flatten().collect::<Vec<String>>();
    let p = input
        .iter()
        .map(|x| x.split(':').collect::<Vec<_>>())
        .map(|x| (x[0], x[1]))
        .map(|(u, x)| (u, x.split('|').collect::<Vec<_>>()))
        .map(|(u, x)| {
            (
                u,
                (
                    x[0].split(' ')
                        .map(str::trim)
                        .filter(|x| !x.is_empty())
                        .collect::<Vec<_>>(),
                    x[1].split(' ')
                        .map(str::trim)
                        .filter(|x| !x.is_empty())
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .map(|(u, (ans, numbers))| (u, numbers.iter().filter(|x| ans.contains(x)).count()))
        .map(|(u, x)| (u.split(' ').last().unwrap().parse::<u32>().unwrap(), x))
        .fold(HashMap::new(), |mut acc, (current, value)| {
            let value: u32 = value.try_into().unwrap();
            acc.entry(current).or_insert(1);
            for target in current + 1..=current + value {
                acc.entry(target).or_insert(1);
                let repeating_count = *acc.get(&current).unwrap();
                let next = acc.get_mut(&target).unwrap();
                *next += repeating_count;
            }
            acc
        })
        .iter()
        .map(|x| x.1)
        .sum::<i32>();
    // .fold(0, |acc, (_, x)| {
    //     let x: u32 = x.try_into().unwrap();
    //     if x > 0 {
    //         acc + 2_i32.pow(x - 1)
    //     } else {
    //         acc
    //     }
    // });
    dbg!(&p);
    println!("Hello, world!");
}
