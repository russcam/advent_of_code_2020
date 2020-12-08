const INPUT: &str = include_str!("../../input/day_5.txt");

fn main() {
    let passes: Vec<BoardingPass> = INPUT.lines().map(|l| l.into()).collect();
    let mut seat_ids: Vec<i32> = passes.iter().map(|p| p.seat_id()).collect();

    let highest_seat = seat_ids.iter().max().unwrap();

    println!("highest seat is {}", highest_seat);

    seat_ids.sort();
    let mut my_seat = 0;

    for (idx, this_seat) in seat_ids.iter().enumerate() {
        if let Some(next_seat) = seat_ids.get(idx + 1) {
            if next_seat - this_seat == 2 {
                my_seat = this_seat + 1;
                break;
            }
        }
    }

    println!("my seat is {}", my_seat);
}

struct BoardingPass<'a> {
    input: &'a str,
}

impl<'a> From<&'a str> for BoardingPass<'a> {
    fn from(input: &'a str) -> Self {
        BoardingPass { input }
    }
}

impl<'a> BoardingPass<'a> {
    pub fn seat_id(&self) -> i32 {
        let mut row: i32 = 0;
        let mut seat: i32 = 0;

        // F and B, L and R are binary encodings e.g. FBFBBFFRLR = 0101100 and 101
        for (idx, c) in self.input.chars().enumerate() {
            let idx = idx as i32;
            match c {
                'B' => row += 1 * 2i32.pow((idx - 6).abs() as u32),
                'R' => seat += 1 * 2i32.pow((idx - 9).abs() as u32),
                _ => (),
            }
        }

        row * 8 + seat
    }
}

#[cfg(test)]
mod tests {
    use crate::BoardingPass;

    #[test]
    fn test() {
        let pass = BoardingPass::from("FBFBBFFRLR");
        assert_eq!(pass.seat_id(), 357);
    }
}
