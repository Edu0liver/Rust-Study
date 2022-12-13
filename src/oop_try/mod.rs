

mod oop_try {

    fn ex_1(){

        struct AvaregedCollection {
            list: Vec<i32>,
            average: f64,
        }

        impl AvaregedCollection {
            fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    },
                    None => None,
                }
            }

            fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }

    }

    pub fn opp_call() {
        ex_1()
    }

}

pub fn oop_try() {
    crate::oop_try::oop_try::opp_call();
}
