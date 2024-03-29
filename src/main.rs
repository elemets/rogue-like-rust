use tcod::colors::*;
use tcod::console::*;

// defining constants
const SCREEN_WIDTH: i32 = 90;
const SCREEN_HEIGHT: i32 = 60;
const LIMIT_FPS: i32 = 20;

struct Game {

    // State here
}

struct Tcod {
    root: Root,
}


fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
    // handle keys
    // You can choose to import packages within functions rather 
    // than in the overall program this makes it a bit faster
    // if you don't call the function the package isn't imported
    // Also makes the code make more sense e.g. you can see what 
    // package is being used for what part of the code 
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    let key = tcod.root.wait_for_keypress(true);
        
    match key {
        Key{
            code: Enter, 
            alt: true,
            ..
        } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }

        Key { code: Escape, .. } => return true, // exiting game

        // if the key matches the code (Up, Down) then the code after the arrow executes
        // the .. after Up indicates that it should ignore all other input and
        // only work if its Up etc.
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, ..} => *player_y += 1,
        Key { code: Right, ..} => *player_x += 1,
        Key { code: Left, ..} => *player_x -= 1,

        _ => {}
         }
        false
}



fn main() {
    
    // The "let" keyword is used to create new variables 
    // here we are creating a new variable called root which contains the game. 
    let root = Root::initializer()
                .font("dejavu12x12_gs_tc.png", FontLayout::Tcod)
                .font_type(FontType::Greyscale)
                .size(SCREEN_WIDTH, SCREEN_HEIGHT)
                .title("Rogue Like")
                .init();

    let mut tcod = Tcod { root };

    // this sets our game windows FPS to be limited at the FPS limit (20)
    tcod::system::set_fps(LIMIT_FPS);
    
    // player starts in the centre of the sc
    // "mut" keyword means that our variables are editable (mutable!)
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

     // this will keep going while the window is open
    // the window_closed() function returns true if the window was closed and false otherwise
    // we use the ! to negate this at the start so it only runs while the window is open
    // This is the game loop
     while !tcod.root.window_closed() {
         // setting the foreground colour to white
        tcod.root.set_default_foreground(WHITE);
        // clear anything previous from frame
        tcod.root.clear();
        // put character at 1 1 
        tcod.root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        // Background flag ignores the background colour
        // Flush draws everything on the window at once 
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit{
            break;
        }
    }
    





}
