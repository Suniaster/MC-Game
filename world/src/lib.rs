
pub mod components;
pub mod scene;

use anymap::{AnyMap, any::Any,};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};

pub struct World{
    entity_allocator: EntityAllocator,

    components: AnyMap,
    resources: AnyMap,
}

pub type ComponentKey = DefaultKey;

impl World {
    
    pub fn new() -> Self {
        Self {
            entity_allocator: EntityAllocator::new(),
            components: AnyMap::new(),
            resources: AnyMap::new()
        }
    }

    pub fn add_component_storage<T: Any>(&mut self) -> &mut Self{
        self.components.insert(ComponentMap::<T>::new());
        self
    }
    
    pub fn add_resource<T: Any>(&mut self, resource: T) -> &mut Self{
        self.resources.insert(resource);
        self
    }
    
    pub fn component_storage_ref<T: Any>(&self) -> Option<& ComponentMap<T>>{
        self.components.get::<ComponentMap<T>>()
    }

    pub fn component_storage_mut<T: Any>(&mut self) -> Option<&mut ComponentMap<T>>{
        self.components.get_mut::<ComponentMap<T>>()
    }

    pub fn resource_ref<T: Any>(&self) -> Option<& T>{
        self.resources.get::<T>()
    }

    pub fn resource_mut<T: Any>(&mut self) -> Option<&mut T>{
        self.resources.get_mut::<T>()
    }

    pub fn build_entity<'a>(&'a mut self) -> BuildComponentMessage<'a>{
        let entity = self.create_entity();
        BuildComponentMessage{world: self, entity}
    }

    pub fn destroy_entity<'a>(&'a mut self, entity: ComponentKey) -> DestroyEntityMessage<'a>{
        self.entity_allocator.deallocate(entity);
        DestroyEntityMessage{world: self, entity}
    }


    fn create_entity(&mut self) -> ComponentKey{
        return self.entity_allocator.allocate()
    }
    fn add_component_to_entity<T: Any>(&mut self, entity: ComponentKey, component: T){
        self.component_storage_mut::<T>()
            .unwrap().map.insert(entity, component);
    }
    fn remove_component_from_entity<T: Any>(&mut self, entity: ComponentKey){
        self.component_storage_mut::<T>()
            .unwrap().map.remove(entity);
    }

    pub fn component_iter_mut<T: Any>(&mut self) -> impl Iterator<Item = (ComponentKey, &mut T)>{
        self.component_storage_mut::<T>().map(|storage| storage.map.iter_mut()).unwrap()
    }
    pub fn component_iter_ref<T: Any>(&self) -> impl Iterator<Item = (ComponentKey, &T)>{
        self.component_storage_ref::<T>().map(|storage| storage.map.iter()).unwrap()
    }

    pub fn iter_comp<T: Any>(&self) -> impl Iterator<Item = (ComponentKey, &T)>{
        let a = self.component_storage_ref::<T>().unwrap();
        return ComponentMapIter {
            map: a,
            keys_iter: Box::new(self.entity_allocator.allocator.iter())
        };
    }
}

struct ComponentMapIter<'a, T>{ 
    map: &'a ComponentMap<T>,
    keys_iter: Box<dyn Iterator<Item = (ComponentKey, &'a ())> + 'a>,
}

impl <'a, T> Iterator for ComponentMapIter<'a, T> {
    type Item = (ComponentKey, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let next_key = self.keys_iter.next();
        if let Some(key) = next_key {
            return Some( (key.0, self.map.map.get(key.0).unwrap()) );
        }
        None
    }
}

impl <'a, T> ComponentMapIter<'a, T>{
    fn zip<U>(&'a mut self, other: ComponentMapIter<'a, U>) -> impl Iterator<Item = (ComponentKey, &'a T, &'a U)> {
        ComponentMapIterZip{
            a: self,
            b: other
        }
    }
}

pub struct ComponentMapIterZip<'a, T, U> {
    a: &'a mut ComponentMapIter<'a, T>,
    b: ComponentMapIter<'a, U>,
}

impl <'a, T, U> Iterator for ComponentMapIterZip<'a, T, U> {
    type Item = (ComponentKey, &'a T, &'a U);
    fn next(&mut self) -> Option<Self::Item> {
        let next_key = self.a.next();
        if let Some(obja) = next_key {
            let key = obja.0;
            let objb = self.b.map.map.get(key).unwrap();
            return Some( (key, obja.1, objb) );
        }
        None
    }
}


pub struct BuildComponentMessage<'a>{
    world: &'a mut World,
    entity: ComponentKey
}

impl<'a> BuildComponentMessage<'a>{
    pub fn with_component<T: Any>(&mut self, component: T) -> &mut Self {
        self.world.add_component_to_entity(self.entity, component);
        self
    }
    pub fn finish(&self) -> ComponentKey{
        self.entity
    }
}

pub struct DestroyEntityMessage<'a>{
    world: &'a mut World,
    entity: ComponentKey
}

impl<'a> DestroyEntityMessage<'a>{
    pub fn with_component<T: Any>(&mut self) -> &mut Self{
        self.world.remove_component_from_entity::<T>(self.entity);
        self
    }
    pub fn finish(&self) -> &World{
        self.world
    }
}

pub struct EntityAllocator {
    pub allocator: SlotMap<DefaultKey, ()>
}

impl EntityAllocator{
    pub fn new() -> Self {
        Self {
            allocator: SlotMap::new()
        }
    }

    pub fn allocate(&mut self) -> DefaultKey {
        self.allocator.insert(())
    }

    pub fn deallocate(&mut self, key: DefaultKey) {
        self.allocator.remove(key);
    }
}

pub struct ComponentMap<T>{
    pub map: SecondaryMap<DefaultKey, T>
}

impl <T> ComponentMap<T>{
    pub fn new() -> Self {
        Self {
            map: SecondaryMap::new()
        }
    }
}