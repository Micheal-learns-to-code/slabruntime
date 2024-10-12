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
pub const CP2101_PARTNUM: u8 = 0x01;
pub const CP2102_PARTNUM: u8 = 0x02;
pub const CP2103_PARTNUM: u8 = 0x03;
pub const CP2104_PARTNUM: u8 = 0x04;
pub const CP2105_PARTNUM: u8 = 0x05;
pub const CP2108_PARTNUM: u8 = 0x08;
pub const CP210X_PARTNUM_CP2102N_QFN28: u8 = 0x20;
pub const CP210X_PARTNUM_CP2102N_QFN24: u8 = 0x21;
pub const CP210X_PARTNUM_CP2102N_QFN20: u8 = 0x22;
pub const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::new(7, 0); // timeout 7000ms
