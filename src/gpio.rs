use cli::GpioState;
use sysfs_gpio::{Direction, Error, Pin};

pub fn set_pin(id: u8, state: GpioState) -> Result<(), Error> {
    let dir = match state {
        GpioState::High => Direction::High,
        GpioState::Low => Direction::Low,
    };

    let pin = Pin::new(id as u64);
    
    pin.export()
        .expect(&format!("export of pin {} failed", id));

    println!("exported pin {}", id);

    pin.set_direction(dir)
        .expect(&format!("setting pin {} to {:?} failed.", id, dir));

    println!("set pin {} to {:?}", id, dir);

    Ok(())
}
