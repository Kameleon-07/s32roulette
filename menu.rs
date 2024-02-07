pub struct Menu {
    visible_options: Vec<String>,
    option_functions: Vec<fn()>,
    counter: u8,
}

pub enum Direction {
    UP,
    DOWN,
}

impl Menu {
    pub fn new(mut visible_options: Vec<&str>, option_functions: Vec<fn()>) -> Self {
        let mut visible_options_string: Vec<String> = Vec::new();
        for element in visible_options.iter_mut() {
            visible_options_string.push(element.to_string());
        };

        Menu {
            visible_options: visible_options_string,
            option_functions: option_functions,
            counter: 0,
        }
    }

    pub fn change(&mut self, direction: Direction) {
        match direction {
            Direction::UP => {
                if self.counter > 0 {
                    self.counter -= 1;
                }
            }
            Direction::DOWN => {
                if self.counter < (self.option_functions.len() - 1) as u8 {
                    self.counter += 1;
                }
            }
        }
    }

    pub fn choose(&self) {
        (self.option_functions[self.counter as usize])();
    }

    pub fn display(&self) {
        for i in 0..self.visible_options.len() {
            if i as u8 == self.counter {
                println!("> {}", self.visible_options[i]);
            } else {
                println!("  {}", self.visible_options[i]);
            }
        }
    }
}
