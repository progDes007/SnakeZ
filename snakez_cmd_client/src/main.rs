use std::sync::mpsc;
use game_backend::Vector2i;
use game_backend::base::Direction;
use game_backend::events::GlobalEvent;
use game_cmd_front::front;


fn main() {

    let mut game = game_backend::Game::new( Vector2i::new(20,20 ));
    // Create a player control channel
    let (user_control_tx, user_control_rx) = mpsc::channel::<Direction>();
    // Register player
    game.register_player(Some(user_control_rx));

    // Create global events channel
    let (global_update_tx, global_update_rx) = mpsc::channel::<GlobalEvent>();
    game.register_global_event_channel(global_update_tx);

    // Create frontend object
    let mut front = front::Front::new(user_control_tx, global_update_rx);

    // Create shutdown channel for game
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
    // Run the game in separate thread
    let join_handle = std::thread::spawn(move || {
        game.game_loop(shutdown_rx);
    });


    // Run the game
    front.run();

    // Send shutdown signal
    let _ = shutdown_tx.send(());

    // And wait for the game to finish
    let _ = join_handle.join();

}
