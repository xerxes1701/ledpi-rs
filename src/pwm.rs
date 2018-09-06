use i2cdev::linux::LinuxI2CDevice;

use pca9685::PCA9685;

pub fn get_pwm_dev() -> PCA9685<LinuxI2CDevice> {
    let i2c = LinuxI2CDevice::new("/dev/i2c-1", 0x40)
        .expect("Could not get i2c-device.");

    let mut pca = PCA9685::new(i2c)
        .expect("Could not get pca9685 adapter.");

    pca.set_pwm_freq(60.0)
        .expect("Could not set pwm frequency.");

    return pca;
}

pub fn set_pin(pin: u8, on: u16, off: u16) {
    let mut pwm = get_pwm_dev();

    pwm.set_pwm(pin, on, off)
        .expect("Could not set pwm pulse.");

    println!("set pwm pin {} to on={}, off={}", pin, on, off);
}