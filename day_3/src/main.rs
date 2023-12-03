fn main() {
    let mut input = std::io::stdin().lines().flatten().collect::<Vec<_>>();
    input.push(String::from("........"));
    let mut v = Vec::new();
    let matcher = |c: char| c.is_ascii() && c != '.' && !c.is_alphanumeric(); // symbols
    let rev_search = |c: &str| {
        let iter = c
            .chars()
            .rev()
            .take_while(|x| x.is_numeric() || matcher(*x));
        if iter.clone().all(|x| !matcher(x)) {
            Some(iter.collect::<String>().chars().rev().collect::<String>())
        } else {
            None
        }
    };
    let search_forward = |c: &str| {
        let iter = c.chars().take_while(|x| x.is_numeric() || matcher(*x));
        if iter.clone().all(|x| !matcher(x)) {
            Some(iter.collect::<String>())
        } else {
            None
        }
    };
    for p in input.windows(2) {
        dbg!(p);
        // current line
        let mut current_line_added = Vec::new();
        for (x, _) in p[0].match_indices(matcher) {
            let left = p[0].chars().nth(x - 1).map(char::is_numeric);
            if left == Some(true) {
                let c = &p[0][..x];
                let element = rev_search(c);
                if let Some(x) = element {
                    current_line_added.push(x);
                }
            }
            let right = p[0].chars().nth(x + 1).map(char::is_numeric);
            if right == Some(true) {
                let c = &p[0][x + 1..];
                let element = search_forward(c);
                if let Some(x) = element {
                    current_line_added.push(x);
                }
            }
        }
        // now vs next line
        for (x, _) in p[0].match_indices(matcher) {
            let down = p[1].chars().nth(x).map(char::is_numeric);
            if down == Some(true) {
                let right = &p[1][x..];
                let left = &p[1][..=x];
                let back = rev_search(left);
                let front = search_forward(right);
                if back.is_none() || front.is_none() {
                    continue;
                }
                let back = back.unwrap();
                let front = front.unwrap();
                let ans = match (back.len(), front.len()) {
                    (0, _) => front,
                    (_, 0) => front,
                    _ => {
                        let mut a = back;
                        a.pop();
                        a + &front
                    }
                };
                v.push(ans);
            } else {
                let left_down = p[1].chars().nth(x - 1).map(char::is_numeric);
                if left_down == Some(true) {
                    let c = &p[1][..x];
                    let element = rev_search(c);
                    if let Some(x) = element {
                        current_line_added.push(x);
                    }
                }
                let right_down = p[1].chars().nth(x + 1).map(char::is_numeric);
                if right_down == Some(true) {
                    let c = &p[1][..x];
                    let element = search_forward(c);
                    if let Some(x) = element {
                        current_line_added.push(x);
                    }
                }
            }
        }
        // next line vs now
        for (x, _) in p[1].match_indices(matcher) {
            let up = p[0].chars().nth(x).map(char::is_numeric);
            if up == Some(true) {
                let right = &p[0][x..];
                let left = &p[0][..=x];
                let back = rev_search(left);
                let front = search_forward(right);

                if back.is_none() || front.is_none() {
                    continue;
                }
                let back = back.unwrap();
                let front = front.unwrap();
                let ans = match (back.len(), front.len()) {
                    (0, _) => front,
                    (_, 0) => front,
                    _ => {
                        let mut a = back;
                        a.pop();
                        a + &front
                    }
                };
                if !current_line_added.contains(&ans) {
                    v.push(ans);
                }
            } else {
                let left_diagnoal = p[0].chars().nth(x - 1).map(char::is_numeric);
                if left_diagnoal == Some(true) {
                    let c = &p[0][..x];
                    let element = rev_search(c);
                    if let Some(element) = element {
                        if !current_line_added.contains(&element) {
                            v.push(element);
                        }
                    }
                }
                let right_diagonal = p[0].chars().nth(x + 1).map(char::is_numeric);
                if right_diagonal == Some(true) {
                    let c = &p[0][x + 1..];
                    let element = search_forward(c);
                    if let Some(element) = element {
                        if !current_line_added.contains(&element) {
                            v.push(element);
                        }
                    }
                }
            }
        }
        v.extend(current_line_added);
        dbg!(&v);
    }

    dbg!(&v);
    let a = v.iter().filter_map(|x| x.parse::<i32>().ok()).sum::<i32>();
    dbg!(a);
    println!("Hello, world!");
}
