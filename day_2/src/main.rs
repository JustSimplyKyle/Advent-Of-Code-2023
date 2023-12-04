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
                    .map(str::trim)
                    .map(|x| x.split(' ').collect::<Vec<_>>())
                    .map(|x| (x[1], x[0]))
                {
                    let value = value.parse::<i32>().unwrap();
                    let p = acc.entry(key).or_insert(value);
                    if *p < value {
                        *p = value;
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
