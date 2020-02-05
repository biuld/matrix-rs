use rand::{thread_rng, Rng};

#[derive(Debug)]
enum SnakeLifeCycle {
    Growing,
    Living,
    Dying,
}

#[derive(Debug)]
pub struct Snake {
    pub max_length: u16,
    pub current: Vec<u16>,
    pub lifetime: u16,
    delay: u16,
    birthplace: u16,
    state: SnakeLifeCycle,
}

impl Snake {
    pub fn new(max_length: u16, lifetime: u16, birthplace: u16) -> Self {
        assert!(max_length > 0);

        Snake {
            state: SnakeLifeCycle::Growing,
            max_length: max_length,
            current: Vec::with_capacity(max_length as usize),
            lifetime: lifetime,
            birthplace: birthplace,
            delay: thread_rng().gen_range(0u16, 50u16),
        }
    }

    fn grow(&mut self) {
        if self.current.is_empty() {
            self.current.push(self.birthplace);
        } else if let Some(&last) = self.current.last().clone() {
            self.current.push(last + 1)
        }
    }

    fn crawl(&mut self) {
        self.current = self.current.iter().map(|x| x + 1).collect()
    }

    fn shorten(&mut self) {
        self.current = self.current.drain(1..).collect();
    }

    pub fn visit(&mut self) -> bool {
        if self.delay != 0 {
            self.delay -= 1;
            return false;
        } else {
            self.develop();
        }

        true
    }

    pub fn develop(&mut self) {
        match self.state {
            SnakeLifeCycle::Growing => {
                self.grow();
                if self.max_length as usize == self.current.len() {
                    self.state = SnakeLifeCycle::Living;
                }
            }
            SnakeLifeCycle::Living => {
                self.crawl();
                if let Some(&last) = self.current.last().clone() {
                    if last == self.lifetime {
                        self.state = SnakeLifeCycle::Dying;
                    }
                }
            }
            SnakeLifeCycle::Dying => {
                self.shorten();
                if self.current.is_empty() {
                    self.state = SnakeLifeCycle::Growing;
                    self.delay = thread_rng().gen_range(0u16, 10u16);
                }
            }
        }
    }
}
