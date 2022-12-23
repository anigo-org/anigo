use crate::{Core, LifeCycle};

impl Core {
    pub fn extend_life_cycle(&mut self, life_cycle: Vec<LifeCycle>) {
        self.life_cycle.lock().unwrap().extend(life_cycle);
    }

    pub fn alter_life_cycle(&mut self, life_cycle: Vec<LifeCycle>) {
        *self.life_cycle.lock().unwrap() = life_cycle;
    }

    pub fn get_life_cycle(&self) -> Vec<LifeCycle> {
        self.life_cycle.lock().unwrap().clone()
    }
}
