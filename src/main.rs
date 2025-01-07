use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::collections::HashSet;
use enigo::{Key, Enigo, Settings, Keyboard, Direction};
use phf::Map;
use phf_macros::phf_map;
use std::{thread, time};

// pin numbers that the rows and columns are connected to on pi
const ROWS: [u8; 10] = [17, 18, 15, 14, 4, 23, 24, 25, 8, 7];
const COLUMNS: [u8; 5] = [22, 27, 10, 9, 11];

/*
NOTE: buttons operate in three different modes. main mode, second mode, and keyboard mode.
button 0 toggles second mode. button 5 toggles kb mode.
while in kb mode, button 0 is replaced with shift and no longer toggles second mode.
every mode is just a different set up key mappings.
*/

/*
TODO: add mappings for second mode.
*/

// key mappings for the main operating mode.
// first value in tuple is text input, second value is a single keystroke that fires after text
// needs to be this way because enigo doesn't allow "special" key inputs while using text function
const MAIN_MODE: Map<u8, (Option<&str>, Option<Key>)> = phf_map! {
    0u8 => (None, None),
    1u8 => (None, Some(Key::F1)),
    2u8 => (None, Some(Key::Escape)),
    3u8 => (None, Some(Key::UpArrow)),
    4u8 => (None, Some(Key::Backspace)),
    5u8 => (None, None),
    6u8 => (Some("x"), None),
    7u8 => (None, Some(Key::LeftArrow)),
    8u8 => (None, Some(Key::F2)),
    9u8 => (None, Some(Key::RightArrow)),
    10u8 => (Some("plot2d()"), Some(Key::LeftArrow)),
    11u8 => (Some("table()"), Some(Key::LeftArrow)),
    12u8 => (None, Some(Key::F3)),
    13u8 => (None, Some(Key::DownArrow)),
    14u8 => (None, Some(Key::F4)),
    15u8 => (None, Some(Key::Tab)),
    16u8 => (Some("sin()"), Some(Key::LeftArrow)),
    17u8 => (Some("cos()"), Some(Key::LeftArrow)),
    18u8 => (Some("tan()"), Some(Key::LeftArrow)),
    19u8 => (Some("round()"), None),
    20u8 => (Some("sqrt()"), Some(Key::LeftArrow)),
    21u8 => (None, Some(Key::F5)),
    22u8 => (None, Some(Key::F7)),
    23u8 => (Some("]"), None),
    24u8 => (Some("^"), None),
    25u8 => (Some("^2"), None),
    26u8 => (None, Some(Key::F6)),
    27u8 => (Some("("), None),
    28u8 => (Some(")"), None),
    29u8 => (Some("/"), None),
    30u8 => (Some("log()"), Some(Key::LeftArrow)),
    31u8 => (Some("7"), None),
    32u8 => (Some("8"), None),
    33u8 => (Some("9"), None),
    34u8 => (Some("*"), None),
    35u8 => (Some("ln()"), Some(Key::LeftArrow)),
    36u8 => (Some("4"), None),
    37u8 => (Some("5"), None),
    38u8 => (Some("6"), None),
    39u8 => (Some("-"), None),
    40u8 => (Some("="), None),
    41u8 => (Some("1"), None),
    42u8 => (Some("2"), None),
    43u8 => (Some("3"), None),
    44u8 => (Some("+"), None),
    45u8 => (None, None),
    46u8 => (Some("0"), None),
    47u8 => (Some("."), None),
    48u8 => (Some(","), None),
    49u8 => (None, Some(Key::Return)),
};

// key mappings for the keyboard mode
const KB_MODE: Map<u8, Key> = phf_map! {
    0u8 => (Key::Shift),
    1u8 => (Key::Unicode('Z')),
    2u8 => (Key::Unicode('A')),
    3u8 => (Key::Unicode('Q')),
    4u8 => (Key::Unicode('1')),
    6u8 => (Key::Unicode('X')),
    7u8 => (Key::Unicode('S')),
    8u8 => (Key::Unicode('W')),
    9u8 => (Key::Unicode('2')),
    10u8 => (Key::Control),
    11u8 => (Key::Unicode('C')),
    12u8 => (Key::Unicode('D')),
    13u8 => (Key::Unicode('E')),
    14u8 => (Key::Unicode('3')),
    15u8 => (Key::Alt),
    16u8 => (Key::Unicode('V')),
    17u8 => (Key::Unicode('F')),
    18u8 => (Key::Unicode('R')),
    19u8 => (Key::Unicode('4')),
    20u8 => (Key::Space),
    21u8 => (Key::Unicode('B')),
    22u8 => (Key::Unicode('G')),
    23u8 => (Key::Unicode('T')),
    24u8 => (Key::Unicode('5')),
    25u8 => (Key::LeftArrow),
    26u8 => (Key::Unicode('N')),
    27u8 => (Key::Unicode('H')),
    28u8 => (Key::Unicode('Y')),
    29u8 => (Key::Unicode('6')),
    30u8 => (Key::UpArrow),
    31u8 => (Key::Unicode('M')),
    32u8 => (Key::Unicode('J')),
    33u8 => (Key::Unicode('U')),
    34u8 => (Key::Unicode('7')),
    35u8 => (Key::DownArrow),
    36u8 => (Key::Unicode(',')),
    37u8 => (Key::Unicode('K')),
    38u8 => (Key::Unicode('I')),
    39u8 => (Key::Unicode('8')),
    40u8 => (Key::RightArrow),
    41u8 => (Key::Unicode('.')),
    42u8 => (Key::Unicode('L')),
    43u8 => (Key::Unicode('O')),
    44u8 => (Key::Unicode('9')),
    45u8 => (Key::Unicode('\'')),
    46u8 => (Key::Unicode('/')),
    47u8 => (Key::Unicode(';')),
    48u8 => (Key::Unicode('P')),
    49u8 => (Key::Unicode('0')),
};

// handler for keypress events
fn key_pressed(key: u8, modes: &mut (bool, bool), enigo: &mut Enigo) {
    match &modes {
        // main mode handling
        (false, false) => {
            let pressed = MAIN_MODE.get(&key).unwrap();
            if pressed.0.is_some() {
                enigo.text(pressed.0.unwrap()).unwrap();
            }
            if pressed.1.is_some() {
                enigo.key(pressed.1.unwrap(), Direction::Click).unwrap();
            }
        }
        // keyboard mode handling
        (_, true) => 'kb: {
            if key == 5 {break 'kb;}
            let pressed = KB_MODE.get(&key).unwrap();
            enigo.key(*pressed, Direction::Press).unwrap();
        }
        _ => ()
    }
}

// handler for key release events (mainly just for kb mode)
fn key_released(key: u8, modes: &mut (bool, bool), enigo: &mut Enigo) {
    match key {
        // second mode toggle (notice how it doesn't change if keyboard mode is active)
        0 => {
            if !modes.1 {modes.0 = !modes.0;}
        },
        // keyboard mode toggle
        5 => {
            modes.1 = !modes.1;
        }
        _ => {
            // i know i don't really need a match statement here, but it might be necessary for second mode. may replace with if statement if it turns out not to be
            match &modes {
                (_, true) => {
                    let released = KB_MODE.get(&key).unwrap();
                    enigo.key(*released, Direction::Release).unwrap();
                }
                _ => ()
            }
        }
    }
}

fn main() {
    let gpio: Gpio = Gpio::new().unwrap();
    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    // turning pin numbers into addressable pins
    let row_pins: [InputPin; 10] = ROWS.map(|x| gpio.get(x).unwrap().into_input_pulldown());
    let mut column_pins: [OutputPin; 5] = COLUMNS.map(|x| gpio.get(x).unwrap().into_output());

    // per polling cycle, set of keys that were pressed previously and set of keys that are currently being pressed
    let mut prev_pressed_keys: HashSet<u8> = HashSet::new();
    let mut curr_pressed_keys: HashSet<u8> = HashSet::new();

    // left is second mode, right is keyboard mode
    let mut modes: (bool, bool) = (false, false);

    loop {
        curr_pressed_keys.clear();
        for (i, column) in &mut column_pins.iter_mut().enumerate() {
            // power every column individually
            column.set_high();
            for (j, row) in row_pins.iter().enumerate() {
                // check each row per column
                if row.is_high() {
                    // add key number to currently pressed keys. top-left is key 0, increasing by 1 from left to right, top to bottom. ends on key 49 in bottom-right
                    curr_pressed_keys.insert((i + 5 * j).try_into().unwrap());
                }
            }
            column.set_low();
        }

        // all of the keys that were pressed this polling cycle but not the last one
        for key_down in curr_pressed_keys.difference(&prev_pressed_keys) {
            key_pressed(*key_down, &mut modes, &mut enigo);
        }
        // all of the keys that were pressed in the previous polling cycle but not this one
        for key_up in prev_pressed_keys.difference(&curr_pressed_keys) {
            key_released(*key_up, &mut modes, &mut enigo);
        }
        prev_pressed_keys = curr_pressed_keys.clone(); // is there a better way to do this?
        thread::sleep(time::Duration::from_millis(2)); // 500hz polling rate
    }
}
