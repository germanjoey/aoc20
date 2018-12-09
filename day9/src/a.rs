struct MarbleCircle {
	current_marble_index: i32,
	marbles: Vec<u32>,
}

impl MarbleCircle {
    fn new() -> MarbleCircle {
		let marbles: Vec<u32> = vec![0];
        return MarbleCircle { marbles: marbles, current_marble_index: 0 };
    }

	fn add_marble(&mut self, new_marble: u32) -> u32 {
		if (new_marble % 23) == 0 {
			return self._add_marble23(new_marble);
		}

		return self._add_marble_normal(new_marble);
	}

	fn _add_marble23(&mut self, new_marble: u32) -> u32 {
		let mut new_marble_index: i32 = self.current_marble_index - 7;
		if new_marble_index < 0 {
			new_marble_index += self.marbles.len() as i32;
		}

		let removed_marble = self.marbles.remove(new_marble_index as usize);
		self.current_marble_index = new_marble_index;
		return new_marble + removed_marble;
	}

	fn _add_marble_normal(&mut self, new_marble: u32) -> u32 {
		let size = self.marbles.len() as i32;
		let mut new_marble_index: i32 = self.current_marble_index + 2;

		if new_marble_index == size {
			self.marbles.push(new_marble);
		}
		else {
			if new_marble_index > size {
				new_marble_index -= size;
			}

			self.marbles.insert(new_marble_index as usize, new_marble);
		}

		self.current_marble_index = new_marble_index;
		return 0;
	}

	fn print_circle (& self) {
		for i in 0..self.marbles.len() {
			if i == (self.current_marble_index as usize) {
				print!("({}) ", self.marbles[i]);
			}
			else {
				print!(" {}  ", self.marbles[i]);
			}
		}
	}

}

struct MarbleGame {
	total_players: u32,
	current_player: u32,
	scores: Vec<u32>,
	marble_circle: MarbleCircle,
}

impl MarbleGame {
    fn new(total_players: u32) -> MarbleGame {
		let marble_circle = MarbleCircle::new();
		let mut scores: Vec<u32> = vec![];
		for _i in 0..(total_players+1) {
			scores.push(0);
		}

		return MarbleGame {
			total_players: total_players, current_player: 0,
			marble_circle: marble_circle, scores: scores
		};
	}

	fn play_next_round (&mut self, new_marble: u32) {
		let result: u32 = self.marble_circle.add_marble(new_marble);
		self.scores[self.current_player as usize] += result;
		self.current_player += 1;
		if self.current_player >= self.total_players {
			self.current_player = 0;
		} 
	}

	fn print_winner (&mut self) {
		let mut max_score = 0;
		let mut max_score_owner = 0;

		for player_id in 0..self.scores.len() {
			let score = self.scores[player_id];
			if score > max_score {
				max_score = score;
				max_score_owner = player_id;
			}
		}

		println!("The winner is: {}, with {} points.", max_score_owner + 1, max_score);
	}

	fn print_status (& self) {
		print!("[{}]: ", self.current_player + 1);
		self.marble_circle.print_circle();
		print!("\n");
	}
}


fn main() {
	let total_marbles = 70918;
	let total_players = 464;

	let mut game = MarbleGame::new(total_players);

	for i in 1..(total_marbles+1) {
		game.play_next_round(i);
	}

	game.print_winner();
}
