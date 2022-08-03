use plugins::{App, Plugin};

pub enum EntityFeatures {
    Render,
    Collide
}

pub struct EntityCreated {
    pub features: Vec<EntityFeatures>,
    pub entity: specs::Entity
}
impl EntityCreated {
    pub fn new(entity: specs::Entity) -> Self {
        Self {
            features: vec![],
            entity
        }
    }
}
    

#[derive(Default)]
pub struct EntityCreatedBuffer(Vec<EntityCreated>);
impl EntityCreatedBuffer {
    pub fn add(&mut self, entity: specs::Entity) -> EntityCreatedBuilder {
        return EntityCreatedBuilder {
            buffer: self,
            v: EntityCreated::new(entity)
        }
    }
}

pub struct EntityCreatedBuilder<'a>{
    buffer: &'a mut EntityCreatedBuffer,
    v: EntityCreated
}

impl<'a> EntityCreatedBuilder<'a> {
    pub fn with_feature(mut self, feature: EntityFeatures) -> Self {
        self.v.features.push(feature);
        self
    }
    pub fn build(self) -> &'a mut EntityCreatedBuffer {
        self.buffer.0.push(self.v);
        self.buffer
    }
}




pub struct EntityEventsPlugin;
impl Plugin for EntityEventsPlugin {
    fn build(&mut self, app: &mut App) {
        app.add_resource(EntityCreatedBuffer);
    }
}