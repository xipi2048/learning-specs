use {
    specs::{
        Builder, Component, DispatcherBuilder, Join, Read, ReadStorage, System,
        VecStorage, World, WriteStorage, Entities
    },
    specs_derive::Component,
};

#[derive(Default)]
struct DeltaTime(f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Name {
    name: String,
}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>, 
        ReadStorage<'a, Position>
    );

    fn run(&mut self, (entities, name_storage, pos): Self::SystemData) {        
        for (ent, pos) in (&*entities, &pos).join() {
            
            //name is optional
            if let Some(name) = name_storage.get(ent) {
                print!("Hello {}! ", &name.name);
            }

            println!("{:?}", &pos);
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;

        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

fn main() {
    //create world
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .with(UpdatePos, "update_pos", &["hello_world"])
        .with(HelloWorld, "hello_updated", &["update_pos"])
        .build();

    dispatcher.setup(&mut world.res);

    world.add_resource(DeltaTime(0.05));

    //create entities with components
    world
        .create_entity()
        // .with(Name {
        //     name: "entity 1".to_string(),
        // })
        .with(Position { x: 4.0, y: 7.0 })
        .build();

    world
        .create_entity()
        .with(Name {
            name: "entity 2".to_string(),
        })
        .with(Position { x: 2.0, y: 5.0 })
        .with(Velocity { x: 0.1, y: 0.2 })
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();
}
