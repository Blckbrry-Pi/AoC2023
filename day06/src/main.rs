fn round_count(time: usize, dist: usize) -> usize {
    let mut count = 0;
    for i in 0..(time+1) {
        let attempt_dist = i * (time - i);
        if attempt_dist > dist {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("./day06/input.txt")
        .expect("Couldn't read the input file");

    let (time_str, dist_str) = input.split_once('\n').unwrap();
    let times: Vec<_> = time_str[10..]
        .split(' ')
        .filter(|val| !val.is_empty())
        .map(|val| val.parse::<usize>().unwrap())
        .collect();
    
    let dists: Vec<_> = dist_str[10..]
        .split(' ')
        .filter(|val| !val.is_empty())
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let time = time_str[10..].split(' ').collect::<String>().parse::<usize>().unwrap();
    let dist = dist_str[10..].split(' ').collect::<String>().parse::<usize>().unwrap();

    let rounds: Vec<_> = times.into_iter().zip(dists).collect();

    let ways_to_win_p1 = rounds.iter().map(|(time, dist)| round_count(*time, *dist)).collect::<Vec<_>>();
    println!("Part 1: {}", ways_to_win_p1.iter().product::<usize>());

    let ways_to_win_p2 = round_count(time, dist);
    println!("Part 2: {}", ways_to_win_p2);

}
