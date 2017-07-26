use std::mem;
use std::thread::spawn;

fn show(table: &Vec<bool>, rows: usize, columns: usize) {
	let mut prepare = "".to_string();
    for r in 0..rows {
        for c in 0..columns {
            match table[c+r*columns] {
                false => prepare.push('x'),
                true => prepare.push('@'),
            }
        }
        prepare.push('\n');
    }
    prepare.push('\n');
	print!("{}", prepare);
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

fn discover(mut table: &mut Vec<bool>, rows: usize, columns: usize) {
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
		show(&table, rows, columns);
	}
}

fn discover_block(queue: Vec<Vec<bool>>, rows: usize, columns: usize) {
	for mut table in queue {
		discover(&mut table, rows, columns);
	}
}

fn main() {
    let rows = 4;
    let columns = 4;
    let mut thread_handles = Vec::new();
    let mut queue = Vec::new();
    let mut table = Vec::with_capacity(rows*columns);
    for _ in 0..(2u32.pow((rows*columns) as u32)) {
		for _ in 0..table.len() {
			if table.pop() == Some(false) {
				table.push(true);
				break;
			}
		}
		for _ in table.len()..table.capacity() {
			table.push(false);
		}
		queue.push(table.clone());
		if (queue.len() as u32) == 2u32.pow((rows*columns) as u32)/4 {
			let queue_cpy = queue.to_vec();
		    thread_handles.push(spawn(|| {
				discover_block(queue_cpy, 4, 4);
			}));
			queue.clear();
		}
	}
	for handle in thread_handles {
		handle.join().unwrap();
	}
}
