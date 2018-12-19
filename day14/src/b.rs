fn next_recipe(current_index: usize, scores: &Vec<u32>) -> usize {
    return ((current_index + 1 + (scores[current_index] as usize)) % scores.len()) as usize
}

fn main() {
    let input = "939601";
    let mut scores = vec![3, 7];
    let mut currents = vec![0, 1];

    loop {
        let new_recipies: Vec<_> = format!(
            "{}",
            currents.iter().map(|&i| scores[i as usize]).sum::<u32>()
        )
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

        for new_recipie in new_recipies {
            if scores.len() > input.len() {
                let suffix = scores[scores.len() - input.len()..]
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("");

                if suffix == input {
                    println!("{:?}", scores.len() - input.len());
                    return;
                }
            }
            scores.push(new_recipie);
        }

        currents = currents.iter().map(|&i| next_recipe(i, &scores)).collect();
    }
}
