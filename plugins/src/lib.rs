use specs::prelude::*;

pub trait PluginSytem<'a> : System<'a> {
    fn name(&self) -> &'static str;
    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }
}

pub trait Plugin {
    fn build(&self, _app: &mut App){}
    fn before_run(&self, _app: &mut App<'static>){}
}

pub struct App<'a> {
    pub world: specs::World,
    plugins: Vec<Box<dyn Plugin>>,
    dispatcher_builder: DispatcherBuilder<'a, 'a>,
    dispatcher: Dispatcher<'a, 'a>,
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

    pub fn with<P: Plugin + 'static>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        let t = Box::new(plugin);
        self.plugins.push(t);
        self
    }

    pub fn add_system<S>(&mut self, system: S) 
    where
        S: for<'c> PluginSytem<'c> + Send + 'a,
    {
        let name = system.name();
        let deps = system.deps();
        self.dispatcher_builder.add(system, name, &deps);
    }

    pub fn add_system_thread_local<S>(&mut self, system: S) 
    where
        S: for<'c> PluginSytem<'c> + Send + 'a,
    {
        self.dispatcher_builder.add_thread_local(system);
    }

    pub fn add_component_storage<C>(&mut self)
    where
        C: Component + Send + Sync + 'a,
        C::Storage: Default,
    {
        self.world.register::<C>();
    }

    pub fn add_resource<R>(&mut self, resource: R)
    where
        R: Resource + Send + Sync + 'a,
    {
        self.world.insert(resource);
    }

    pub fn setup(&mut self) {
        let b = std::mem::replace(&mut self.dispatcher_builder, DispatcherBuilder::new());
        let disp = b.build();
        self.dispatcher = disp;
    }

    pub fn run_once(&mut self) {
        self.dispatcher.dispatch(&mut self.world);
    }
}

impl App<'static> {
    pub fn run(&mut self) {
        let plugins = std::mem::replace(&mut self.plugins, Vec::new());
        for plugin in plugins.iter() {
            plugin.before_run(self);
        }
        let _ = std::mem::replace(&mut self.plugins, plugins);
        self.run_once();
    }
}
/*********** TESTS *************/

struct TestSystem;
impl System<'_> for TestSystem {
    type SystemData = ();
    fn run(&mut self, _data: Self::SystemData) {
        println!("TestSystem aaaaaa");
    }
}
impl PluginSytem<'_> for TestSystem {
    fn name(&self) -> &'static str {
        "TestSystem"
    }
}

struct TestComponent;
impl Component for TestComponent {
    type Storage = VecStorage<Self>;
}

pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TestSystem);
        app.add_component_storage::<TestComponent>();
    }
}