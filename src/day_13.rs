pub fn part_1(input: &str) -> usize {
    input.split("\n\n").map(|pattern| {      
        let (rows, cols) = get_pattern(pattern);
        reflection_idx(&cols).unwrap_or_else(|| reflection_idx(&rows).unwrap() * 100)
    }).sum()
}

pub fn part_2(input: &str) -> usize {
    input.split("\n\n").map(|pattern| {
        let (rows, cols) = get_pattern(pattern);
        smudge_reflection_idx(&cols).unwrap_or_else(|| smudge_reflection_idx(&rows).unwrap() * 100)
    }).sum()
}

fn reflection_idx(seq: &[u32]) -> Option<usize> {
    (0..seq.len()-1).find_map(|idx| {
        if (0..idx+1)
            .into_iter()
            .rev()
            .zip(idx+1..seq.len())
            .all(|(i, j)| seq[i] == seq[j]) 
        {
            Some(idx + 1)
        } else {
            None
        }
   })
}

fn smudge_reflection_idx(seq: &[u32]) -> Option<usize> {
    (0..seq.len()-1).find_map(|idx| {
        if (0..idx+1)
            .into_iter()
            .rev()
            .zip(idx+1..seq.len())
            .fold(0, |acc, (i, j)| acc + (seq[i] ^ seq[j]).count_ones())
        ==  1 {
            Some(idx + 1)
        } else {
            None
        }
    })
}

fn get_pattern(p: &str) -> (Vec<u32>, Vec<u32>) {
    let mut cols = vec![0_u32; p.split_once("\n").unwrap().0.len()];
    let rows: Vec<u32> = p.lines().enumerate().map(|(j, s)| {
        let mut n = 0_u32;
        s.chars().enumerate().for_each(|(i, ch)| match ch {
            '#' => {
                n |= 1 << i;
                cols[i] |= 1 << j;
            },
            '.' => {},
            _ => unreachable!(),
        });
        n
    }).collect();
    (rows, cols)
}