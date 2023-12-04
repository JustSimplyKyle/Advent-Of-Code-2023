use std::ops::ControlFlow;
fn rev_search(c: &str) -> Option<String> {
    let iter = c
        .chars()
        .rev()
        .take_while(|x| x.is_numeric() || symbol_matcher(*x));
    if iter.clone().all(|x| !symbol_matcher(x)) {
        Some(iter.collect::<String>().chars().rev().collect::<String>())
    } else {
        None
    }
}
fn symbol_matcher(c: char) -> bool {
    c.is_ascii() && c != '.' && !c.is_alphanumeric()
}
fn search_forward(c: &str) -> Option<String> {
    let iter = c
        .chars()
        .take_while(|x| x.is_numeric() || symbol_matcher(*x));
    if iter.clone().all(|x| !symbol_matcher(x)) {
        Some(iter.collect::<String>())
    } else {
        None
    }
}

fn main() {
    let mut input = std::io::stdin().lines().flatten().collect::<Vec<_>>();
    input.push(String::from("........"));
    let mut v = Vec::new();
    for p in input.windows(3) {
        for (x, _) in p[1].match_indices(symbol_matcher) {
            let mut now = Vec::new();
            current_line(&p[1], x, &mut now);
            if let ControlFlow::Break(_) = now_vs_prev(&p[0], x, &mut now) {
                continue;
            }
            if let ControlFlow::Break(_) = now_vs_next(&p[2], x, &mut now) {
                continue;
            }
            // comment out for first day
            if now.len() == 2 {
                v.push(now);
            }
            // v.push(now);
        }
    }

    dbg!(&v);
    let a = v
        .iter()
        .map(|vec| {
            vec.iter()
                .filter_map(|y| y.parse::<i32>().ok())
                // .sum::<i32>()
                .product::<i32>()
        })
        .sum::<i32>();
    dbg!(a);
    println!("Hello, world!");
}

fn current_line(p: &String, x: usize, now: &mut Vec<String>) {
    let left = p.chars().nth(x - 1).map(char::is_numeric);
    if left == Some(true) {
        let c = &p[..x];
        let element = rev_search(c);
        if let Some(x) = element {
            now.push(x);
        }
    }
    let right = p.chars().nth(x + 1).map(char::is_numeric);
    if right == Some(true) {
        let c = &p[x + 1..];
        let element = search_forward(c);
        if let Some(x) = element {
            now.push(x);
        }
    }
}

fn now_vs_prev(dest: &String, pos: usize, v: &mut Vec<String>) -> ControlFlow<()> {
    let up = dest.chars().nth(pos).map(char::is_numeric);
    if up == Some(true) {
        let right = &dest[pos..];
        let left = &dest[..=pos];
        let back = rev_search(left);
        let front = search_forward(right);

        if back.is_none() || front.is_none() {
            return ControlFlow::Break(());
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
        let left_diagnoal = dest.chars().nth(pos - 1).map(char::is_numeric);
        if left_diagnoal == Some(true) {
            let c = &dest[..pos];
            let element = rev_search(c);
            if let Some(element) = element {
                v.push(element);
            }
        }
        let right_diagonal = dest.chars().nth(pos + 1).map(char::is_numeric);
        if right_diagonal == Some(true) {
            let c = &dest[pos + 1..];
            let element = search_forward(c);
            if let Some(element) = element {
                v.push(element);
            }
        }
    }
    ControlFlow::Continue(())
}

fn now_vs_next(dest: &String, pos: usize, v: &mut Vec<String>) -> ControlFlow<()> {
    let down = dest.chars().nth(pos).map(char::is_numeric);
    if down == Some(true) {
        let right = &dest[pos..];
        let left = &dest[..=pos];
        let back = rev_search(left);
        let front = search_forward(right);
        if back.is_none() || front.is_none() {
            return ControlFlow::Break(());
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
        let left_down = dest.chars().nth(pos - 1).map(char::is_numeric);
        if left_down == Some(true) {
            let c = &dest[..pos];
            let element = rev_search(c);
            if let Some(x) = element {
                v.push(x);
            }
        }
        let right_down = dest.chars().nth(pos + 1).map(char::is_numeric);
        if right_down == Some(true) {
            let c = &dest[pos + 1..];
            let element = search_forward(c);
            if let Some(x) = element {
                v.push(x);
            }
        }
    }
    ControlFlow::Continue(())
}
