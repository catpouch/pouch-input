use rppal::gpio::{Gpio, OutputPin};

const ROWS: [u8; 10] = [7, 8, 25, 24, 23, 4, 14, 15, 18, 17];
const COLUMNS: [u8; 5] = [22, 27, 10, 9, 11];

fn main() {
    let gpio = Gpio::new().unwrap();
    let row_pins: [OutputPin; 10] = ROWS.map(|x| gpio.get(x).unwrap().into_output());
    let mut column_pins: [OutputPin; 5] = COLUMNS.map(|x| gpio.get(x).unwrap().into_output());
    loop {
        for column in &mut column_pins {
            column.set_high();
            for row in &row_pins {
                if row.is_set_high() {
                    let pin_num = row.pin();
                    println!("{pin_num}");
                }
            }
            column.set_low();
        }
    }
}
