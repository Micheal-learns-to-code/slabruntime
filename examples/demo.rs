
/*
 * 
 * This file is part of the rust-cp210x-runtime project.
 * 
 * Licensed under the MIT License. You may obtain a copy of the License at
 * 
 *     https://opensource.org/licenses/MIT
 * 
 * This code is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement.
 * 
 * Note: This project is not affiliated with, endorsed by, or in any way associated with Silicon Labs. For more information about Silicon Labs and their products, please visit their official website at https://www.silabs.com.
 */
use slabruntime::Cp210xRuntime;

fn main() {
    println!("CP210xRuntime Example!");
    let cp210x_runtime = Cp210xRuntime::new().expect("Something wrong with the library: ");
    match cp210x_runtime.cp210x_rt_get_num_devices() {
        Ok(num_dev) => println!("Number of CP210x device: {} ", num_dev),
        Err(err) => println!("{}", err),
    }

    let h = match cp210x_runtime.cp210x_rt_open(0) {
        Ok(h) => Some(h),
        Err(e) => {
            println!("Can not open device {}", e);
            None
        }
    };

    if let Some(handle) = h {
        match cp210x_runtime.cp210x_rt_get_device_product_string(&handle, true) {
            Ok(s) => println!("Device product string : {}", s),
            Err(e) => println!("Can not get device product string {}", e),
        }

        match cp210x_runtime.cp210x_rt_get_device_product_string(&handle, false) {
            Ok(s) => println!("Device product string in Unicode : {}", s),
            Err(e) => println!("Can not get device product string in Unicode {}", e),
        }

        match cp210x_runtime.cp210x_rt_get_device_serial_string(&handle, true) {
            Ok(s) => println!("Device serial string : {}", s),
            Err(e) => println!("Can not get device serial string {}", e),
        }

        match cp210x_runtime.cp210x_rt_get_device_serial_string(&handle, false) {
            Ok(s) => println!("Device serial string in Unicode : {}", s),
            Err(e) => println!("Can not get device serial string in Unicode {}", e),
        }

        match cp210x_runtime.cp210x_rt_get_receiver_max_timeout(&handle) {
            Ok(max_timeout) => println!("Receiver max timeout {} ", max_timeout),
            Err(e) => println!("Can not get receiver max timeout {}", e),
        }

        let max_timeout = 2000;
        match cp210x_runtime.cp210x_rt_set_receiver_max_timeout(&handle, max_timeout) {
            Ok(_) => println!("Max timeout set correctly {} ", max_timeout),
            Err(e) => println!("Can not set receiver max timeout {} ", e),
        }

        match cp210x_runtime.cp210x_rt_get_receiver_max_timeout(&handle) {
            Ok(max_timeout) => println!("Receiver max timeout set successfully {} ", max_timeout),
            Err(e) => println!("Can not get receiver max timeout {}", e),
        }
    }
}
