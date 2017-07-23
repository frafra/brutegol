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
    let mut table = vec![
        false,  true, false, false,
        false, false,  true, false,
         true,  true,  true, false,
        false, false, false, false,
    ];
    assert!(rows*columns == table.len());
    show(&table, rows, columns);
    for _ in 0..5 {
        println!();
        next(&mut table, rows, columns);
        show(&table, rows, columns);
    }
}
