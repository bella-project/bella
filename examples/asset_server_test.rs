use bella::assets::TextFile;
use bella::prelude::*;

fn start(mut instance: ResMut<Instance>) {
    println!(
        "{:?}",
        instance
            .asset_server()
            .load_file::<TextFile>("examples/assets/message.txt")
            .unwrap()
    );
}

fn main() {
    App::new("Asset Server Test", 1280, 720)
        .new_world()
        .on_start(start)
        .run();
}
