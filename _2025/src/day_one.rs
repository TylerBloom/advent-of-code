

pub fn problem_a() {
    static DATA: &str = include_str!("../input/day_one_a.txt");
    let (pw, _) = DATA.lines().fold((0_i32, 50_i32), |(mut pw, mut acc), line| {
        let mut chars = line.trim().chars();
        match chars.next().unwrap() {
            'L' => acc -= chars.as_str().parse::<i32>().unwrap(),
            'R' => acc += chars.as_str().parse::<i32>().unwrap(),
            c => panic!("Unknown starting char: {c:?}"),
        }
        acc = acc.rem_euclid(100);
        pw += (acc == 0) as i32;
        (pw, acc)
    });
    println!("The password is: {pw}");
}
