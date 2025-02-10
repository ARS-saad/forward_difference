use std::collections::HashMap;

#[derive(Debug)]
pub struct ArrName {
    pub values: Vec<i32>,
    pub has_map: HashMap<i32, Vec<i32>>,
}

impl ArrName {
    pub fn new(f: Vec<i32>) -> ArrName {
        ArrName {
            values: f,
            has_map: HashMap::new(),
        }
    }

    pub fn forword(&mut self) {
        let mut key = 0_i32;
        let kay: Vec<i32> = self.values.clone();
        self.has_map.insert(key, kay);

        loop {
            let vec = self.has_map.get(&key).unwrap();
            let end_value = vec.len();
            if end_value == 1 {
                break;
            }

            let mut creat = Vec::with_capacity(end_value - 1);
            for (index, &item) in vec.iter().enumerate() {
                if index + 1 == end_value {
                    break;
                }
                creat.push(vec[index + 1] - item);
            }
            key += 1;
            self.has_map.insert(key, creat);
        }
        println!("Reselt: {}", self.has_map.get(&key).unwrap()[0]);
    }

    pub fn display(&self) {
        for index in 0..self.has_map.len() {
            let iter: i32 = index as i32;
            println!("{:?}", self.has_map.get(&iter).unwrap())
        }
    }
}
