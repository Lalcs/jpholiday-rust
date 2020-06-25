use super::holidays::Holiday;

pub struct Registry<'a> {
    registry: Vec<Box<dyn Holiday + 'a>>
}

impl<'a> Registry<'a> {
    pub fn new() -> Self {
        Registry {
            registry: Vec::new()
        }
    }

    pub fn get_registry(&self) -> &Vec<Box<dyn Holiday + 'a>> {
        return &self.registry;
    }

    pub fn register<T: 'a + Holiday>(&mut self, holiday: T) {
        self.registry.push(Box::new(holiday))
    }
}
