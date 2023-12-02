use anyhow::Result;
const VALID_LETTERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn convert_to_string_integers(input: &str) -> String {
    match &input[..] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        x if x.parse::<i32>().is_ok() => x,
        _ => panic!("fuck"),
    }
    .to_string()
}
fn main() -> Result<()> {
    let mut input = Vec::new();
    for t in std::io::stdin().lines() {
        input.push(t?);
    }
    let ans = input.iter().fold(0, |acc, x| {
        let mut all_numbers = Vec::new();
        let number_matching = x.match_indices(char::is_numeric).collect::<Vec<_>>();
        if !number_matching.is_empty() {
            all_numbers.extend(number_matching);
        }
        for word in VALID_LETTERS.iter() {
            let english_matching = x.match_indices(word).collect::<Vec<_>>();
            if !english_matching.is_empty() {
                all_numbers.extend(english_matching);
            }
        }
        all_numbers.sort_by_key(|x| x.0);
        let all_numbers = all_numbers
            .into_iter()
            .map(|x| convert_to_string_integers(x.1))
            .collect::<Vec<_>>();
        let first = all_numbers.first().unwrap().to_owned();
        let last = all_numbers.last().unwrap();
        let ans = (first + last).parse::<i32>().unwrap();
        acc + ans
    });
    println!("{ans}");
    Ok(())
}
