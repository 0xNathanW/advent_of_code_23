use std::cmp::Ordering;

pub fn part_1(input: &str) -> u64 {
    let mut hands: Vec<(&str, u64)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap(), 
                split.next().unwrap().parse::<u64>().unwrap()
            )
        }).collect();

    hands.sort_unstable_by(|a, b| cmp_hands(a.0, b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.1 * (i as u64 + 1))
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut hands: Vec<(&str, u64)> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap(), 
                split.next().unwrap().parse::<u64>().unwrap()
            )
        }).collect();

    hands.sort_unstable_by(|a, b| cmp_hands_2(a.0, b.0));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.1 * (i as u64 + 1))
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

fn char_to_rank(c: char) -> u8 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}

fn cmp_hands(a: &str, b: &str) -> Ordering {
    let rank_a = rank_hand(a);
    let rank_b = rank_hand(b);
    if rank_a == rank_b {
        for (i, j) in a.chars().zip(b.chars()) {
            if i == j {
                continue;
            }
            return char_to_rank(i).cmp(&char_to_rank(j));
        }
        Ordering::Equal
    } else {
        rank_a.cmp(&rank_b)
    }
}

fn rank_hand(hand: &str) -> Rank {
    let mut rank_to_count = [0_u8; 13];
    let mut count_to_rank = [0_u64; 6];
    for card in hand.chars() {
        let rank = char_to_rank(card);
        rank_to_count[rank as usize] += 1;
    }
    for (rank, &count) in rank_to_count.iter().enumerate() {
        count_to_rank[count as usize] |= 1 << rank;
    }
    if count_to_rank[5] != 0 {
        Rank::FiveKind
    } else if count_to_rank[4] != 0 {
        Rank::FourKind
    } else if count_to_rank[3] != 0 && count_to_rank[3].count_ones() == 2 {
        Rank::FullHouse
    } else if count_to_rank[3] != 0 && count_to_rank[2] != 0 {
        Rank::FullHouse
    } else if count_to_rank[3] != 0 {
        Rank::ThreeKind
    } else if count_to_rank[2].count_ones() >= 2 {
        Rank::TwoPair
    } else if count_to_rank[2] == 0 {
        Rank::HighCard
    } else {
        Rank::OnePair
    }
}

fn char_to_rank_2(c: char) -> i8 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => -1,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => unreachable!(),
    }
}


fn cmp_hands_2(a: &str, b: &str) -> Ordering {
    let rank_a = rank_hand_2(a);
    let rank_b = rank_hand_2(b);
    if rank_a == rank_b {
        for (i, j) in a.chars().zip(b.chars()) {
            if i == j {
                continue;
            }
            return char_to_rank_2(i).cmp(&char_to_rank_2(j))
        }
        Ordering::Equal
    } else {
        rank_a.cmp(&rank_b)
    }
}

fn rank_hand_2(hand: &str) -> Rank {
    let mut rank_to_count = [0_u8; 13];
    let mut count_to_rank = [0_u64; 6];

    let mut num_jokers = 0;
    for card in hand.chars() {
        if card == 'J' {
            num_jokers += 1;
        } else {
            let rank = char_to_rank_2(card);
            rank_to_count[rank as usize] += 1;
        }
    }

    if num_jokers > 0 {
        let mut best_rank_joker = 0;
        let mut max_count = 0;
        for (rank, &count) in rank_to_count.iter().enumerate() {
            if count + num_jokers > max_count {
                max_count = count + num_jokers;
                best_rank_joker = rank;
            }
        }
        rank_to_count[best_rank_joker] += num_jokers;
    }
    for (rank, &count) in rank_to_count.iter().enumerate() {
        count_to_rank[count as usize] |= 1 << rank;
    }
    if count_to_rank[5] != 0 {
        Rank::FiveKind
    } else if count_to_rank[4] != 0 {
        Rank::FourKind
    } else if count_to_rank[3] != 0 && count_to_rank[3].count_ones() == 2 {
        Rank::FullHouse
    } else if count_to_rank[3] != 0 && count_to_rank[2] != 0 {
        Rank::FullHouse
    } else if count_to_rank[3] != 0 {
        Rank::ThreeKind
    } else if count_to_rank[2].count_ones() >= 2 {
        Rank::TwoPair
    } else if count_to_rank[2] == 0 {
        Rank::HighCard
    } else {
        Rank::OnePair
    }
}
