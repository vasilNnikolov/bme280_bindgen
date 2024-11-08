pub mod bindings;

#[cfg(test)]
mod tests {
    use crate::bindings;

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

    #[test]
    fn test_read_reg() {
        let device = bindings::bme280_dev {
            chip_id: 0,
            intf: bindings::bme280_intf_BME280_I2C_INTF,
            intf_rslt: 0,
            intf_ptr: &a as std::os::raw::c_void,
        };
    }
}
