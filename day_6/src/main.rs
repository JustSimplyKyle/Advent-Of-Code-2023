fn main() {
    let input = std::io::stdin()
        .lines()
        .flatten()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    let p = input
        .chunks(2)
        .map(|x| (&x[0], &x[1]))
        .map(|(time, distance)| {
            (
                time.split(':').collect::<Vec<_>>(),
                distance.split(':').collect::<Vec<_>>(),
            )
        })
        .map(|(time, distance)| {
            (
                (
                    time[0],
                    time[1]
                        .split_whitespace()
                        // .collect::<Vec<_>>()
                        .fold(String::new(), |acc, x| acc + x),
                ),
                (
                    distance[0],
                    distance[1]
                        .split_whitespace()
                        // .collect::<Vec<_>>()
                        .fold(String::new(), |acc, x| acc + x),
                ),
            )
        })
        .map(|(time, distance)| {
            (
                time.1.parse::<i64>().unwrap(),
                distance.1.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    // .fold(Vec::new(), |mut acc, (time, distance)| {
    //     for x in time.1.iter().zip(distance.1.iter()) {
    //         let time = x.0.parse::<i32>().unwrap();
    //         let distance = x.1.parse::<i32>().unwrap();
    //         acc.push((time, distance));
    //     }
    //     acc
    // });
    let ans: f64 = p
        .iter()
        .map(|x| (x.0 as f64, x.1 as f64))
        .map(|(curr, record)| {
            let pos: f64 = ((-curr + (curr.powi(2) - 4.0 * record).sqrt()) / 2.0) - 1.0;
            let neg: f64 = ((-curr - (curr.powi(2) - 4.0 * record).sqrt()) / 2.0) + 1.0;
            let ans = (pos.ceil() - neg.floor()).abs() + 1.0;
            ans
        })
        .product();
    dbg!(ans);
    println!("Hello, world!");
}
