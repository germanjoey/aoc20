fn next_recipe(current_index: usize, scores: &Vec<u32>) -> usize {
    return ((current_index + 1 + (scores[current_index] as usize)) % scores.len()) as usize
}

fn main() {
    let input = 939601;
    let mut scores = vec![3, 7];
    let mut currents = vec![0, 1];

    while scores.len() < input + 10 {
        let new_recipies: Vec<_> = format!(
            "{}",
            currents.iter().map(|&i| scores[i as usize]).sum::<u32>()
        )
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
        scores.extend(new_recipies.into_iter());

        currents = currents.iter().map(|&i| next_recipe(i, &scores)).collect();
    }
    
    let output = scores[input..input + 10].iter().map(|x| format!("{}", x));

    println!("{:?}", output);
}
