use std::mem;

fn show(table: &Vec<bool>, rows: usize, columns: usize) {
    for r in 0..rows {
        for c in 0..columns {
            match table[c+r*columns] {
                false => print!("□"),
                true => print!("■"),
            }
        }
        println!();
    }
}

fn next(table: &mut Vec<bool>, rows: usize, columns: usize) {
    let mut next = Vec::with_capacity(table.len());
    let mut sum: u8;
    for i in 0..table.len() {
        sum = 0;
        if i%columns != 0 {
			sum += table[i-1] as u8;
		}
        if (i+1)%columns != 0 {
			sum += table[i+1] as u8;
		}
        if i >= columns {
			if i%columns != 0 {
				sum += table[i-columns-1] as u8;
			}
			sum += table[i-columns] as u8;
			if (i+1)%columns != 0 {
				sum += table[i-columns+1] as u8;
			}
	    }
        if i < table.len()-columns {
			if i%columns != 0 {
				sum += table[i+columns-1] as u8;
			}
			sum += table[i+columns] as u8;
			if (i+1)%columns != 0 {
				sum += table[i+columns+1] as u8;
			}
	    }
        next.push(match sum {
            2 => table[i],
            3 => true,
            _ => false,
        });
    }
    mem::swap(table, &mut next);
}

fn main() {
    let rows = 4;
    let columns = 4;
    let mut table = Vec::with_capacity(rows*columns);
    for i in 0..(2u32.pow((rows*columns) as u32)) {
		let s = format!("{:01$b}", i, rows*columns);
		for j in 0..s.len() {
		match s.chars().nth(j).unwrap() {
			 '0' => table.push(false),
			 '1' => table.push(true),
			 _ => {},
		 }
		}
		let mut history = Vec::new();
		history.push(table.to_vec());
		let mut repeated: i8;
		loop {
			repeated = -1;
			next(&mut table, rows, columns);
			'comparison: for i in 0..history.len() {
				for j in 0..table.len() {
					if history[i][j] != table[j]  {
						continue 'comparison;
					}
				}
				repeated = i as i8;
				break;
			}
			if repeated >= 0 {
				break;
			}
			history.push(table.to_vec());
		}
		if repeated == 0 {
			println!("");
			show(&table, rows, columns);
		}
		table.clear();
	}
}
