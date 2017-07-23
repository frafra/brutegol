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
    let mut next = Vec::with_capacity(table.len());;
    let mut sum: u8;
    for i in 0..table.len() {
        sum = 0;
        for s1 in 0..3 {
            match s1 {
                0 if i%columns == 0 => continue,
                2 if (i+1)%columns == 0 => continue,
                _ => {},
            }
            for s2 in 0..3 {
                match s2 {
                    0 if i < columns => continue,
                    2 if i >= columns*(rows-1) => continue,
                    1 if s1 == 1 => continue,
                    _ => {},
                }
                sum += table[i+s1-1+s2*columns-columns] as u8;
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
