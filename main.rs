fn max_sequence(seq: &[i32]) -> i32 
{
    if seq.is_empty() { return 0; }
    
    let all_pos = seq.iter().all(|&x| x > 0);
    if all_pos == true  { return seq.iter().sum::<i32>(); }
    
    let all_neg = seq.iter().all(|&x| x < 0);
    if all_neg == true  { return 0; }
    
    
    let mut sums: Vec<i32> = Vec::new();
    for sw_len in 1..=seq.len()
    {
        let sums_sw: Vec<i32> = seq.windows(sw_len)
                                .map(|window| window.iter().sum())
                                .collect(); 
        sums.extend(sums_sw);
    }

    *sums.iter().max().unwrap() as i32
}
    
    


fn main ()
{
    println!("{:?}", max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}
