use std::fmt::{Display, Formatter, Result};

struct Table {
    table: Vec<Vec<usize>>,
}

impl Table {
    fn new(num1: usize, num2: usize) -> Self {
        let mut table: Vec<Vec<usize>> = Vec::with_capacity(num1 as usize);

        for i in 0..(num1 + 1) {
            let mut vec_to_push = Vec::new();
            vec_to_push.push(i);

            if i == 0 {
                for j in 1..(num2 + 1) {
                    vec_to_push.push(j);
                    continue;
                }
            }

            table.push(vec_to_push);
        }

        let c_cmp = table[0].clone();

        for (index, array) in table.iter_mut().enumerate() {
            if index == 0 {continue};

            for i in 1..(num2 + 1) {
                let value = c_cmp[i] * index;

                array.push(value)
            }
        }

        Table { table }
    }
}

// Format Table Visualization
impl Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Ok(
            for vec in self.table.iter() {
                for i in vec.iter() {
                    write!(f, "{} ", i);
                }

                write!(f, "\n");
            }
        )
    }
}

pub fn exec() {
    let table = Table::new(9, 9);
    println!("{}", table);
}