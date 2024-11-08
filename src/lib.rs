pub mod bindings;

unsafe extern "C" fn my_read_reg(
    reg_addr: u8,
    reg_data: *mut u8,
    len: u32,
    intf_ptr: *mut ::std::os::raw::c_void,
) -> i8 {
    println!("reading register {}, n_bytes {}", reg_addr, len);
    *reg_data = 5;
    if len > 1 {
        *(reg_data.add(1)) = 6;
    }
    0
}
unsafe extern "C" fn my_write_reg(
    reg_addr: u8,
    reg_data: *const u8,
    len: u32,
    intf_ptr: *mut ::std::os::raw::c_void,
) -> i8 {
    println!("writing to register {}", reg_addr);
    0
}

unsafe extern "C" fn my_delay(period: u32, intf_ptr: *mut ::std::os::raw::c_void) {
    println!("delaying by {} us", period);
}

pub fn new_bme() -> bindings::bme280_dev {
    let calib_data = bindings::bme280_calib_data {
        dig_t1: 0,
        dig_t2: 0,
        dig_t3: 0,
        dig_p1: 0,
        dig_p2: 0,
        dig_p3: 0,
        dig_p4: 0,
        dig_p5: 0,
        dig_p6: 0,
        dig_p7: 0,
        dig_p8: 0,
        dig_p9: 0,
        dig_h1: 0,
        dig_h2: 0,
        dig_h3: 0,
        dig_h4: 0,
        dig_h5: 0,
        dig_h6: 0,
        t_fine: 0,
    };
    let device = bindings::bme280_dev {
        chip_id: 0,
        intf: bindings::bme280_intf_BME280_I2C_INTF,
        intf_ptr: std::ptr::null_mut(),
        intf_rslt: 0,
        read: Some(my_read_reg),
        write: Some(my_write_reg),
        delay_us: Some(my_delay),
        calib_data,
    };
    device
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::*;
    #[test]
    fn test_read_reg() {
        let mut dev = new_bme();
        unsafe {
            bindings::bme280_init(&mut dev);
        }
    }
}
