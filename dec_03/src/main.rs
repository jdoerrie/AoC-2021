use std::io::BufRead;

fn main() {
    let nums: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| usize::from_str_radix(&line.unwrap(), 2).unwrap())
        .collect();

    let mut oxy_mask = 0usize;
    let mut co2_mask = 0usize;

    for i in (0..usize::BITS).rev() {
        let oxy_acc = nums
            .iter()
            .filter(|&num| {
                num.checked_shr(i + 1).unwrap_or(0) == oxy_mask.checked_shr(i + 1).unwrap_or(0)
            })
            .map(|num| (num & (1 << i)) >> i)
            .fold([0; 2], |mut accum, i| {
                accum[i] += 1;
                accum
            });
        oxy_mask |= ((oxy_acc[0] <= oxy_acc[1]) as usize) << i;

        let co2_acc = nums
            .iter()
            .filter(|&num| {
                num.checked_shr(i + 1).unwrap_or(0) == co2_mask.checked_shr(i + 1).unwrap_or(0)
            })
            .map(|num| (num & (1 << i)) >> i)
            .fold([0; 2], |mut accum, i| {
                accum[i] += 1;
                accum
            });
        let mut argmin = (co2_acc[0] > co2_acc[1]) as usize;
        if co2_acc[argmin] == 0 {
            argmin = 1 - argmin;
        }
        co2_mask |= argmin << i;
    }

    println!("{} * {} = {}", oxy_mask, co2_mask, oxy_mask * co2_mask);
}
