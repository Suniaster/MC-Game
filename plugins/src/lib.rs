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
    pub dispatcher: Dispatcher<'a, 'a>,
}
trait NewTrait: Plugin + Sized {}
impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            world: specs::World::new(),
            plugins: Vec::new(),
            dispatcher_builder: DispatcherBuilder::new(),
            dispatcher: DispatcherBuilder::new().build(),
        }
    }

    pub fn with(&mut self, plugin: &dyn Plugin) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn add_system<S>(&mut self, system: S) 
    where
        S: for<'c> PluginSytem<'c> + Send + 'a,
    {
        self.dispatcher_builder.add(system, "", &[]);
    }

    pub fn setup(&mut self) {
        let b = std::mem::replace(&mut self.dispatcher_builder, DispatcherBuilder::new());
        let disp = b.build();
        self.dispatcher = disp;
    }

    pub fn run(&mut self) {
        self.setup();
        self.dispatcher.dispatch(&mut self.world);
    }
}


struct TestSystem;
impl System<'_> for TestSystem {
    type SystemData = ();
    fn run(&mut self, _data: Self::SystemData) {
        println!("TestSystem aaaaaa");
    }
}
impl PluginSytem<'_> for TestSystem {
    fn name(&self) -> String {
        "TestSystem".to_string()
    }
}

pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TestSystem);
        // app.builder.borrow_mut().with(TestSystem, "test_system", &[]);
    }
}