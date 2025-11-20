use std::io;

fn manacher(s: &str) -> Vec<usize> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    let mut t = Vec::with_capacity(2 * n + 1);
    t.push('#');
    for &c in &chars {
        t.push(c);
        t.push('#');
    }

    let m = t.len();
    let mut p = vec![0; m];
    let mut center = 0;
    let mut right = 0;

    for i in 0..m {
        if i < right {
            let mirror = 2 * center as isize - i as isize;
            p[i] = (right - i).min(p[mirror as usize]);
        }

        while i >= p[i] + 1 && i + p[i] + 1 < m && t[i - p[i] - 1] == t[i + p[i] + 1] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }

    p
}

fn find_longest_palindrome(s: &str) -> String {
    let p = manacher(s);
    let (center, radius_ref) = p.iter().enumerate().max_by_key(|(_, r)| *r).unwrap();

    let radius = *radius_ref;

    let start = (center - radius) / 2;
    s.chars().skip(start).take(radius).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input = input.trim().to_string();
    let result = find_longest_palindrome(input.as_str());
    println!("{}", result);
}
