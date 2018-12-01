#[cfg(test)]
extern crate quickcheck;
extern crate num_cpus;

use std::env;
use std::mem;
use std::thread;

pub fn show(table: &Vec<bool>, rows: usize, columns: usize) -> String {
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
    return prepare;
}

pub fn next(table: &mut Vec<bool>, _rows: usize, columns: usize) {
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

pub fn discover(mut table: &mut Vec<bool>, rows: usize, columns: usize) -> i8 {
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
    if repeated == 0 && history.len() > 1 {
        show(&table, rows, columns);
    }
    return repeated;
}

fn discover_block(queue: Vec<Vec<bool>>, rows: usize, columns: usize) {
    for mut table in queue {
        discover(&mut table, rows, columns);
    }
}

fn mirror_horizontal(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    return i%columns+(rows-1-i/columns)*columns;
}

fn mirror_vertical(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    return i+columns-2*(i%columns)-1;
}

fn mirror_diagonal(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    if rows == 1 {
        return i;
    }
    return i%columns*rows+i/columns;
}

fn mirror_diagonal2(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    let p = mirror_vertical(len, rows, columns, i);
    return mirror_diagonal(len, rows, columns, p);
}

fn rotate_180(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    return len-1-i;
}

fn rotate_90(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    let p = mirror_diagonal(len, rows, columns, i);
    return mirror_vertical(len, rows, columns, p);
}

fn rotate_270(len: usize, rows: usize, columns: usize, i: usize) -> usize {
    let p = mirror_diagonal2(len, rows, columns, i);
    return mirror_vertical(len, rows, columns, p);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} rows columns", args[0]);
        return;
    }
    let rows: usize = args[1].parse().unwrap();
    let columns: usize = args[2].parse().unwrap();
    let num_cpus = num_cpus::get();
    let mut thread_handles = Vec::new();
    let mut thread:thread::JoinHandle<_>;
    let mut queue = Vec::new();
    let mut table = Vec::with_capacity(rows*columns);
    let mut p: usize;
    let mut transformations: Vec<fn(usize, usize, usize, usize) -> usize> = vec![
        rotate_180,
        mirror_horizontal,
        mirror_vertical,
    ];
    if rows == columns {
        transformations.push(mirror_diagonal);
        transformations.push(mirror_diagonal2);
        transformations.push(rotate_90);
        transformations.push(rotate_270);
    }
    'generate: for _ in 0..(2u32.pow((rows*columns) as u32)) {
        for _ in 0..table.len() {
            if table.pop() == Some(false) {
                table.push(true);
                break;
            }
        }
        for _ in table.len()..table.capacity() {
            table.push(false);
        }
        for transformation in transformations.iter() {
            for i in 0..table.len() {
                p = transformation(table.len(), rows, columns, i);
                if table[i] > table[p] {
                    continue 'generate;
                } else if table[i] < table[p] {
                    break;
                }
            }
        }
        queue.push(table.clone());
        if queue.len() > 2usize.pow(16) {
            thread_handles.push(thread::spawn(move || {
                discover_block(queue, rows, columns);
            }));
            if thread_handles.len() == num_cpus {
                thread = thread_handles.remove(0);
                thread.join().expect("Unable to join the thread");
            }
            queue = Vec::new();
        }
    }
    discover_block(queue, rows, columns); // Remaining tables
    for handle in thread_handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod test {
    use quickcheck::*;
    use super::*;

    fn glider() -> Vec<bool> {
        return vec![false,  true, false, false,
                    false, false,  true, false,
                     true,  true,  true, false,
                    false, false, false, false];
    }

    #[test]
    fn next_test() {
        let mut table = glider();
        let result = vec![false, false, false, false,
                           true, false,  true, false,
                          false,  true,  true, false,
                          false,  true, false, false];
        next(&mut table, 4, 4);
        for i in 0..table.len() {
            assert_eq!(table[i], result[i]);
        }
    }

    #[test]
    fn discover_test() {
        assert_eq!(discover(&mut glider(), 4, 4), 7);
    }

    #[test]
    fn show_test() {
        let result = "x@xx\nxx@x\n@@@x\nxxxx\n\n";
        assert_eq!(show(&mut glider(), 4, 4), result);
    }

    quickcheck! {
        fn rotate_180_test(rows: usize, columns: usize, i: usize) -> bool {
            if i >= rows*columns {
                return true;
            }
            let mut pos = i;
            for _ in 0..2 {
                pos = rotate_180(rows*columns, rows, columns, pos);
            }
            return i == pos;
        }
    }

    quickcheck! {
        fn mirror_horizontal_test(rows: usize, columns: usize, i: usize) -> bool {
            if i >= rows*columns {
                return true;
            }
            let mut pos = i;
            for _ in 0..2 {
                pos = mirror_horizontal(rows*columns, rows, columns, pos);
            }
            return i == pos;
        }
    }

    quickcheck! {
        fn mirror_horizontal_vertical(rows: usize, columns: usize, i: usize) -> bool {
            if i >= rows*columns {
                return true;
            }
            let mut pos = i;
            for _ in 0..2 {
                pos = mirror_vertical(rows*columns, rows, columns, pos);
            }
            return i == pos;
        }
    }

    quickcheck! {
        fn mirror_diagonal_test(size: usize, i: usize) -> bool {
            if i >= size*size {
                return true;
            }
            let mut pos = i;
            for _ in 0..2 {
                pos = mirror_diagonal(size*size, size, size, pos);
            }
            return i == pos;
        }
    }
}
