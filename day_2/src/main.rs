use std::collections::HashMap;

fn main() {
    let input: Vec<String> = std::io::stdin().lines().flatten().collect();
    let c = input
        .iter()
        .map(|x| x.split(':').collect::<Vec<_>>())
        .map(|x| (x.first().unwrap().to_owned(), x.last().unwrap().to_owned()))
        .map(|(u, x)| (u, x.split(';').collect::<Vec<_>>())) // groups
        .map(|(u, x)| {
            x.iter().fold((u, HashMap::new()), |(u, mut acc), x| {
                for (key, value) in x
                    .split(',')
                    .map(|x| x.trim_start().trim_end())
                    .map(|x| x.split(' ').collect::<Vec<_>>())
                    .map(|x| (x[1], x[0]))
                {
                    let value = value.parse::<i32>().unwrap();
                    if let Some(p) = acc.iter_mut().find(|x| x.0 == &key) {
                        if *p.1 < value {
                            *p.1 = value;
                        }
                    } else {
                        acc.insert(key, value);
                    }
                }
                (u, acc)
            })
        });
    let ans: i32 = c
        // .filter(|(_, x)| {
        //     x.get("blue").unwrap() <= &14
        //         && x.get("red").unwrap() <= &12
        //         && x.get("green").unwrap() <= &13
        // })
        // .map(|x| x.0.matches(char::is_numeric).collect::<String>())
        // .filter_map(|x| x.parse::<i32>().ok())
        .inspect(|x| {
            dbg!(x);
        })
        .map(|x| x.1.values().product::<i32>())
        .sum();
    println!("{ans}");
}
