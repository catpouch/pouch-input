use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::collections::HashSet;
use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use phf::Map;
use phf_macros::phf_map;
use std::{thread, time};

// pin numbers that the rows and columns are connected to on pi
const ROWS: [u8; 10] = [17, 18, 15, 14, 4, 23, 24, 25, 8, 7];
const COLUMNS: [u8; 5] = [22, 27, 10, 9, 11];

const KEY_LEFTPAREN: u16 = 65535;
const KEY_RIGHTPAREN: u16 = 65534;
const KEY_CARET: u16 = 65533;

/*
NOTE: buttons operate in three different modes. main mode, second mode, and keyboard mode.
button 0 toggles second mode. button 5 toggles kb mode.
while in kb mode, button 0 is replaced with shift and no longer toggles second mode.
every mode is just a different set of key mappings.
*/

/*
TODO: add mappings for second mode.
*/

// key mappings for the main operating mode.
// first value in tuple is text input, second value is a single keystroke that fires after text
// needs to be this way because enigo doesn't allow "special" key inputs while using text function
const MAIN_MODE: Map<u8, &[u16]> = phf_map! {
    0u8 => (&[]),
    1u8 => (&[KEY_F1]),
    2u8 => (&[KEY_ESC]),
    3u8 => (&[KEY_UP]),
    4u8 => (&[KEY_BACKSPACE]),
    5u8 => (&[]),
    6u8 => (&[KEY_X]),
    7u8 => (&[KEY_LEFT]),
    8u8 => (&[]),
    9u8 => (&[KEY_RIGHT]),
    10u8 => (&[KEY_P, KEY_L, KEY_O, KEY_T, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    11u8 => (&[]),
    12u8 => (&[KEY_F3]),
    13u8 => (&[KEY_DOWN]),
    14u8 => (&[KEY_F4]),
    15u8 => (&[KEY_TAB]),
    16u8 => (&[KEY_S, KEY_I, KEY_N, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    17u8 => (&[KEY_C, KEY_O, KEY_S, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    18u8 => (&[KEY_T, KEY_A, KEY_N, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    19u8 => (&[KEY_R, KEY_O, KEY_U, KEY_N, KEY_D, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    20u8 => (&[KEY_S, KEY_Q, KEY_R, KEY_T, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    21u8 => (&[KEY_F5]),
    22u8 => (&[KEY_F7]),
    23u8 => (&[]),
    24u8 => (&[KEY_CARET]),
    25u8 => (&[KEY_CARET, KEY_2]),
    26u8 => (&[KEY_F6]),
    27u8 => (&[KEY_LEFTPAREN]),
    28u8 => (&[KEY_RIGHTPAREN]),
    29u8 => (&[KEY_SLASH]),
    30u8 => (&[KEY_L, KEY_O, KEY_G, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    31u8 => (&[KEY_7]),
    32u8 => (&[KEY_8]),
    33u8 => (&[KEY_9]),
    34u8 => (&[KEY_KPASTERISK]),
    35u8 => (&[KEY_L, KEY_N, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    36u8 => (&[KEY_4]),
    37u8 => (&[KEY_5]),
    38u8 => (&[KEY_6]),
    39u8 => (&[KEY_MINUS]),
    40u8 => (&[KEY_EQUAL]),
    41u8 => (&[KEY_1]),
    42u8 => (&[KEY_2]),
    43u8 => (&[KEY_3]),
    44u8 => (&[KEY_KPPLUS]),
    45u8 => (&[]),
    46u8 => (&[KEY_10]),
    47u8 => (&[KEY_DOT]),
    48u8 => (&[KEY_COMMA]),
    49u8 => (&[KEY_ENTER]),
};

const SECOND_MODE: Map<u8, &[u16]> = phf_map! {
    0u8 => (&[]),
    1u8 => (&[KEY_F1]),
    2u8 => (&[KEY_ESC]),
    3u8 => (&[KEY_UP]),
    4u8 => (&[KEY_BACKSPACE]),
    5u8 => (&[]),
    6u8 => (&[KEY_X]),
    7u8 => (&[KEY_LEFT]),
    8u8 => (&[]),
    9u8 => (&[KEY_RIGHT]),
    10u8 => (&[KEY_P, KEY_L, KEY_O, KEY_T, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    11u8 => (&[]),
    12u8 => (&[KEY_F3]),
    13u8 => (&[KEY_DOWN]),
    14u8 => (&[KEY_F4]),
    15u8 => (&[KEY_TAB]),
    16u8 => (&[KEY_A, KEY_S, KEY_I, KEY_N, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    17u8 => (&[KEY_A, KEY_C, KEY_O, KEY_S, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    18u8 => (&[KEY_A, KEY_T, KEY_A, KEY_N, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    19u8 => (&[KEY_R, KEY_O, KEY_U, KEY_N, KEY_D, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    20u8 => (&[KEY_N, KEY_R, KEY_T, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    21u8 => (&[KEY_F5]),
    22u8 => (&[KEY_F7]),
    23u8 => (&[]),
    24u8 => (&[KEY_P, KEY_I]),
    25u8 => (&[KEY_CARET, KEY_2]),
    26u8 => (&[KEY_KPASTERISK, KEY_1, KEY_10, KEY_CARET]),
    27u8 => (&[KEY_LEFTPAREN]),
    28u8 => (&[KEY_RIGHTPAREN]),
    29u8 => (&[KEY_E]),
    30u8 => (&[KEY_L, KEY_O, KEY_G, KEY_LEFTPAREN, KEY_RIGHTPAREN, KEY_LEFT]),
    31u8 => (&[KEY_7]),
    32u8 => (&[KEY_8]),
    33u8 => (&[KEY_9]),
    34u8 => (&[KEY_KPASTERISK]),
    35u8 => (&[KEY_E, KEY_CARET]),
    36u8 => (&[KEY_4]),
    37u8 => (&[KEY_5]),
    38u8 => (&[KEY_6]),
    39u8 => (&[KEY_MINUS]),
    40u8 => (&[KEY_EQUAL]),
    41u8 => (&[KEY_1]),
    42u8 => (&[KEY_2]),
    43u8 => (&[KEY_3]),
    44u8 => (&[KEY_KPPLUS]),
    45u8 => (&[]),
    46u8 => (&[KEY_10]),
    47u8 => (&[KEY_DOT]),
    48u8 => (&[KEY_COMMA]),
    49u8 => (&[KEY_ENTER]),
};

const DOOM_MODE: Map<u8, u16> = phf_map! {
    0u8 => (0),
    1u8 => (KEY_LEFTCTRL),
    2u8 => (KEY_ESC),
    3u8 => (KEY_UP),
    4u8 => (0),
    5u8 => (0),
    6u8 => (KEY_LEFTSHIFT),
    7u8 => (KEY_LEFT),
    8u8 => (KEY_SPACE),
    9u8 => (KEY_RIGHT),
    10u8 => (0),
    11u8 => (0),
    12u8 => (0),
    13u8 => (KEY_DOWN),
    14u8 => (0),
    15u8 => (0),
    16u8 => (0),
    17u8 => (0),
    18u8 => (0),
    19u8 => (0),
    20u8 => (0),
    21u8 => (0),
    22u8 => (0),
    23u8 => (0),
    24u8 => (0),
    25u8 => (0),
    26u8 => (0),
    27u8 => (0),
    28u8 => (0),
    29u8 => (0),
    30u8 => (0),
    31u8 => (0),
    32u8 => (0),
    33u8 => (0),
    34u8 => (0),
    35u8 => (0),
    36u8 => (0),
    37u8 => (0),
    38u8 => (0),
    39u8 => (0),
    40u8 => (0),
    41u8 => (0),
    42u8 => (0),
    43u8 => (0),
    44u8 => (0),
    45u8 => (0),
    46u8 => (0),
    47u8 => (0),
    48u8 => (0),
    49u8 => (KEY_ENTER),
};

// key mappings for the keyboard mode
const KB_MODE: Map<u8, u16> = phf_map! {
    0u8 => (KEY_LEFTSHIFT),
    1u8 => (KEY_Z),
    2u8 => (KEY_A),
    3u8 => (KEY_Q),
    4u8 => (KEY_1),
    6u8 => (KEY_X),
    7u8 => (KEY_S),
    8u8 => (KEY_W),
    9u8 => (KEY_2),
    10u8 => (KEY_LEFTCTRL),
    11u8 => (KEY_C),
    12u8 => (KEY_D),
    13u8 => (KEY_E),
    14u8 => (KEY_3),
    15u8 => (KEY_LEFTALT),
    16u8 => (KEY_V),
    17u8 => (KEY_F),
    18u8 => (KEY_R),
    19u8 => (KEY_4),
    20u8 => (KEY_SPACE),
    21u8 => (KEY_B),
    22u8 => (KEY_G),
    23u8 => (KEY_T),
    24u8 => (KEY_5),
    25u8 => (KEY_ENTER),
    26u8 => (KEY_N),
    27u8 => (KEY_H),
    28u8 => (KEY_Y),
    29u8 => (KEY_6),
    30u8 => (KEY_BACKSPACE),
    31u8 => (KEY_M),
    32u8 => (KEY_J),
    33u8 => (KEY_U),
    34u8 => (KEY_7),
    35u8 => (KEY_EQUAL),
    36u8 => (KEY_COMMA),
    37u8 => (KEY_K),
    38u8 => (KEY_I),
    39u8 => (KEY_8),
    40u8 => (KEY_MINUS),
    41u8 => (KEY_DOT),
    42u8 => (KEY_L),
    43u8 => (KEY_O),
    44u8 => (KEY_9),
    46u8 => (KEY_SLASH),
    47u8 => (KEY_APOSTROPHE),
    48u8 => (KEY_P),
    49u8 => (KEY_10),
};

// handler for keypress events
fn key_pressed(key: u8, modes: &mut (bool, bool, bool), device: &mut VirtualDevice) {
    println!("{:?}", modes);

    match &modes {
        // main mode handling
        (false, false, false) => {
            let pressed = MAIN_MODE.get(&key).unwrap();
            for key in *pressed {
                match key {
                    &KEY_LEFTPAREN => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_9);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    &KEY_RIGHTPAREN => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_10);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    &KEY_CARET => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_6);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    _ => { let _ = device.click(*key); }
                }
            }
        },
        // second mode
        (true, false, false) => {
            let pressed = SECOND_MODE.get(&key).unwrap();
            for key in *pressed {
                match key {
                    &KEY_LEFTPAREN => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_9);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    &KEY_RIGHTPAREN => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_10);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    &KEY_CARET => {
                        let _ = device.press(KEY_LEFTSHIFT);
                        let _ = device.click(KEY_6);
                        let _ = device.release(KEY_LEFTSHIFT);
                    }
                    _ => { let _ = device.click(*key); }
                }
            }
        }
        (_, _, true) => {
            let pressed = DOOM_MODE.get(&key).unwrap();
            let _ = device.press(*pressed);
        }
        // keyboard mode handling
        (_, true, false) => 'kb: {
            if key == 5 {break 'kb;}
            let pressed = KB_MODE.get(&key).unwrap();
            let _ = device.press(*pressed);
        }
        _ => ()
    }
}

// handler for key release events (mainly just for kb mode)
fn key_released(key: u8, modes: &mut (bool, bool, bool), device: &mut VirtualDevice) {
    match key {
        // second mode toggle (notice how it doesn't change if keyboard mode is active)
        0 if !modes.1 => {
            modes.0 = !modes.0;
        },
        21 if modes.1 => { // DOOM MODE!!!!!
            modes.2 = !modes.2;
        }
        // keyboard mode toggle
        5 => {
            modes.1 = !modes.1;
        }
        _ => {
            // i know i don't really need a match statement here, but it might be necessary for second mode. may replace with if statement if it turns out not to be
            match &modes {
                (_, _, true) => {
                    let released = DOOM_MODE.get(&key).unwrap();
                    let _ = device.release(*released);
                }
                (_, true, false) => {
                    let released = KB_MODE.get(&key).unwrap();
                    let _ = device.release(*released);
                },
                _ => ()
            }
        }
    }
}

fn main() {
    let gpio: Gpio = Gpio::new().unwrap();
    // let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let mut device = VirtualDevice::default().unwrap();

    // turning pin numbers into addressable pins
    let row_pins: [InputPin; 10] = ROWS.map(|x| gpio.get(x).unwrap().into_input_pulldown());
    let mut column_pins: [OutputPin; 5] = COLUMNS.map(|x| gpio.get(x).unwrap().into_output());

    // per polling cycle, set of keys that were pressed previously and set of keys that are currently being pressed
    let mut prev_pressed_keys: HashSet<u8> = HashSet::new();
    let mut curr_pressed_keys: HashSet<u8> = HashSet::new();

    // left is second mode, right is keyboard mode, third is doom mode
    let mut modes: (bool, bool, bool) = (false, false, false);

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

                // if row.is_high() && j == 0 {
                //     println!("{}", i)
                // } else if j == 0 {
                //     println!("nothin")
                // }
            }
            column.set_low();
        }

        // println!("START 1");
        // println!("{:#?}", curr_pressed_keys);
        // println!("{:#?}", prev_pressed_keys);
        // println!("{:#?}", curr_pressed_keys.difference(&prev_pressed_keys));
        // println!("END\n");

        // all of the keys that were pressed this polling cycle but not the last one
        for key_down in curr_pressed_keys.difference(&prev_pressed_keys) {
            println!("{}", key_down);
            key_pressed(*key_down, &mut modes, &mut device);
        }
        // all of the keys that were pressed in the previous polling cycle but not this one
        for key_up in prev_pressed_keys.difference(&curr_pressed_keys) {
            key_released(*key_up, &mut modes, &mut device);
        }

        // println!("START 2");
        // println!("{:#?}", curr_pressed_keys);
        // println!("{:#?}", prev_pressed_keys);
        // println!("{:#?}", curr_pressed_keys.difference(&prev_pressed_keys));
        // println!("END\n");

        prev_pressed_keys = curr_pressed_keys.clone();

        // println!("START 3");
        // println!("{:#?}", curr_pressed_keys);
        // println!("{:#?}", prev_pressed_keys);
        // println!("END\n");

        thread::sleep(time::Duration::from_millis(2)); // 500hz polling rate
    }
}
