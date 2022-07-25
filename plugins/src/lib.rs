use specs::prelude::*;

pub trait PluginSytem<'a> : System<'a> {
    fn name(&self) -> String;
}

pub trait Plugin {
    fn build(&self, app: &mut App);
}

pub struct App<'a> {
    pub world: specs::World,
    pub plugins: Vec<Box<dyn Plugin>>,
    pub dispatcher_builder: DispatcherBuilder<'a, 'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            world: specs::World::new(),
            plugins: Vec::new(),
            dispatcher_builder: DispatcherBuilder::new(),
        }
    }


    pub fn add_system<S>(&mut self, system: S) 
    where
        S: for<'c> PluginSytem<'c> + Send + 'a,
    {
        self.dispatcher_builder.add(system, "", &[]);
    }

    pub fn setup(&mut self) {
        let disp = self.dispatcher_builder.build();
    }

    pub fn run(&mut self) {
        // let mut dispatcher = self.build_systems();
        // dispatcher.dispatch(&mut self.world);
    }
}

fn build_dispatcher<'a>(app: &mut App, plugins: &Vec<Box<dyn Plugin>>) -> Dispatcher<'a, 'a> {
    let mut dispatcher_builder = DispatcherBuilder::new();
    for plugin in plugins {
        plugin.build(app);
    }
    dispatcher_builder.build()
}
struct TestSystem;
impl System<'_> for TestSystem {
    type SystemData = ();
    fn run(&mut self, _data: Self::SystemData) {
        println!("TestSystem");
    }
}

struct TestPlugin;
impl Plugin for TestPlugin {
    // fn name(&self) -> String {
    //     "TestPlugin".to_string()
    // }

    fn build(&self, app: &mut App) {
        // app.builder.borrow_mut().with(TestSystem, "test_system", &[]);
    }
}