use std::fs;

fn main() {
    let data = fs::read_to_string("data/problem08.txt").expect("Could not find input file");
    let grid: Vec<Vec<u8>> = data
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - 48).collect())
        .collect();
    /* ------ Part 1 ------ */
    let row_scan: Vec<Vec<bool>> = grid
        .iter()
        .rev()
        .skip(1)
        .rev()
        .skip(1)
        .map(|l| scan_line(l.iter().cloned()))
        .collect();
    let col_scan: Vec<Vec<bool>> = (1..(grid[0].len() - 1))
        .map(|i| scan_line(grid.iter().map(|l| l[i])))
        .collect();
    let scan_count: usize = (0..row_scan.len())
        .map(|i| {
            (0..col_scan.len())
                .filter(|j| row_scan[i][*j] || col_scan[*j][i])
                .count()
        })
        .sum();
    let count = 2 * grid.len() + 2 * (grid[0].len() - 2) + scan_count;
    println!("Part 1) The count is: {count}");
    /* ------ Part 2 ------ */
    let row_scan: Vec<Vec<usize>> = grid
        .iter()
        .map(|l| (0..l.len()).map(|i| scenic_score(i, l)).collect())
        .collect();
    let col_scan: Vec<Vec<usize>> = (0..grid.len())
        .map(|i| {
            let col: Vec<_> = (0..grid[i].len()).map(|j| grid[j][i]).collect();
            (0..col.len()).map(|i| scenic_score(i, &col)).collect()
        })
        .collect();
    let count: usize = (0..row_scan.len())
        .map(|i| {
            (0..col_scan.len())
                .map(|j| row_scan[i][j] * col_scan[j][i])
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    println!("Part 2) The count is: {count}");
}

fn scan_line<I>(mut line: I) -> Vec<bool>
where
    I: Clone + ExactSizeIterator + DoubleEndedIterator + Iterator<Item = u8>,
{
    let is_visible = |t: u8, max: &mut u8| {
        if t > *max {
            *max = t;
            true
        } else {
            false
        }
    };
    let mut iter = line.clone().rev();
    let mut max = iter.next().unwrap();
    let scan_right: Vec<_> = iter.map(move |t| is_visible(t, &mut max)).collect();
    let mut max = line.next().unwrap();
    let scan_left: Vec<_> = line.map(move |t| is_visible(t, &mut max)).collect();
    scan_left
        .into_iter()
        .zip(scan_right.into_iter().rev().skip(1))
        .map(|(l, r)| l || r)
        .collect()
}

fn scenic_score(pos: usize, line: &Vec<u8>) -> usize {
    let val = line[pos];
    let find = |(c, i): (usize, usize), latest: &mut usize| {
        *latest = c + 1;
        (val <= line[i]).then_some(*latest)
    };
    let mut latest = 0;
    let left_count = (0..pos)
        .rev()
        .enumerate()
        .find_map(|v| find(v, &mut latest))
        .unwrap_or(latest);
    latest = 0;
    let right_count = ((pos + 1)..line.len())
        .enumerate()
        .find_map(|v| find(v, &mut latest))
        .unwrap_or(latest);
    //println!("{val} in {line:?} -> left: {left_count}, right: {right_count}");
    left_count * right_count
}
