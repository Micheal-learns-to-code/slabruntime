use rusb::{Context, Device, DeviceHandle, DeviceList, Direction, Recipient, RequestType};

use crate::errors::Result;
use crate::{constants::*, errors::CP210xError};

/// CP210x runtime
pub struct Cp210xRuntime {
    context: Context,
}

/// CP210x runtime library implementation
impl Cp210xRuntime {
    /// Initialize the CP210x runtime with a new context
    pub fn new() -> Result<Self> {
        Ok(Cp210xRuntime {
            context: Context::new()?,
        })
    }

    /// Determines the number of CP210x devices connected to the system
    pub fn cp210x_rt_get_num_devices(&self) -> Result<u8> {
        let device_list = DeviceList::new_with_context(self.context.clone())?;

        let mut num_devices: u8 = 0;
        for device in device_list.iter() {
            if self.is_cp210x(&device) {
                num_devices += 1;
            }
        }

        Ok(num_devices)
    }

    // Determines if the device is a CP210x device
    fn is_cp210x(&self, device: &Device<Context>) -> bool {
        let device_desc;
        match device.device_descriptor() {
            Ok(dev_desc) => device_desc = dev_desc,
            Err(_) => return false,
        }

        if rusb::constants::LIBUSB_CLASS_PER_INTERFACE == device_desc.class_code() {
            let i_manfacturer: u8;
            match device_desc.manufacturer_string_index() {
                None => return false,
                Some(i) => i_manfacturer = i,
            }
            let i_product: u8;
            match device_desc.product_string_index() {
                None => return false,
                Some(i) => i_product = i,
            }
            let i_serial: u8;
            match device_desc.serial_number_string_index() {
                None => return false,
                Some(i) => i_serial = i,
            }
            if 1 == i_manfacturer && 2 == i_product && 3 <= i_serial {
                let config_desc;
                match device.config_descriptor(0) {
                    Ok(conf_desc) => config_desc = conf_desc,
                    Err(_) => return false,
                }

                if 1 <= config_desc.num_interfaces() {
                    if let Some(interface) = config_desc.interfaces().next() {
                        if let Some(interface_desc) = interface.descriptors().next() {
                            if rusb::constants::LIBUSB_CLASS_VENDOR_SPEC
                                == interface_desc.class_code()
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Opens and returns a handle to a device using a device index determined by the number returned
    /// from cp210x_rt_get_num_devices().
    pub fn cp210x_rt_open(&self, device_index: usize) -> Result<DeviceHandle<Context>> {
        let device_list;
        match DeviceList::new_with_context(self.context.clone()) {
            Ok(dev_list) => device_list = dev_list,
            Err(_) => return Err(CP210xError::CommandFailed),
        }

        let num_devices = device_list.len();

        if device_index < num_devices {
            let mut count = 0;
            for device in device_list.iter() {
                if self.is_cp210x(&device) {
                    if count == device_index {
                        match device.open() {
                            Ok(h) => return Ok(h),
                            Err(_) => return Err(CP210xError::CommandFailed),
                        }
                    } else {
                        count += 1;
                    }
                }
            }
        }
        Err(CP210xError::DeviceNotFound)
    }

    /// Gets the part number of the current device
    pub fn cp210x_rt_get_part_number(&self, handle: &DeviceHandle<Context>) -> Result<u8> {
        let request_type =
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        let request = 0xFF;
        let value = 0x370B;
        let index = 0x0000;
        let part_num: &mut [u8; 1] = &mut [0; 1];
        let timeout = std::time::Duration::new(7000, 0);
        match handle.read_control(request_type, request, value, index, part_num, timeout) {
            Ok(_) => Ok(part_num[0]),
            Err(_) => return Err(CP210xError::DeviceIoFailed),
        }
    }

    // determines if the device is a CP2102N device
    fn is_cp2102n(&self, handle: &DeviceHandle<Context>) -> Result<bool> {
        let part_num = self.cp210x_rt_get_part_number(handle)?;
        Ok(part_num == CP210X_PARTNUM_CP2102N_QFN20
            || part_num == CP210X_PARTNUM_CP2102N_QFN24
            || part_num == CP210X_PARTNUM_CP2102N_QFN28)
    }

    /// Reads and returns the Receiver Max Timeout directly from the device
    pub fn cp210x_rt_get_receiver_max_timeout(
        &self,
        handle: &DeviceHandle<Context>,
    ) -> Result<u16> {
        if !self.is_cp2102n(handle)? {
            return Err(CP210xError::FunctionNotSupported);
        }

        let request_type =
            rusb::request_type(Direction::In, RequestType::Vendor, Recipient::Device);
        let request = 0x18;
        let value = 0x0000;
        let index = 0x0000;
        let max_timeout: &mut [u8; 2] = &mut [0u8; 2];
        let timeout = std::time::Duration::new(7, 0);
        match handle.read_control(request_type, request, value, index, max_timeout, timeout) {
            Ok(_) => Ok(u16::from_ne_bytes(*max_timeout)),
            Err(_) => Err(CP210xError::CommandFailed),
        }
    }

    /// Sets the Receiver Max Timeout directly to the device.
    pub fn cp210x_rt_set_receiver_max_timeout(
        &self,
        handle: &DeviceHandle<Context>,
        max_timeout: u16,
    ) -> Result<u16> {
        if !self.is_cp2102n(handle)? {
            return Err(CP210xError::FunctionNotSupported);
        }

        let request_type =
            rusb::request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
        let request = 0x17;
        let value: [u8; 0] = [];
        let index = 0x0000;
        let timeout = std::time::Duration::new(7, 0);
        match handle.write_control(request_type, request, max_timeout, index, &value, timeout) {
            Ok(_) => Ok(max_timeout),
            Err(_) => Err(CP210xError::CommandFailed),
        }
    }

    /// Gets the product string in the current device
    pub fn cp210x_rt_get_device_product_string(
        &self,
        handle: &DeviceHandle<Context>,
        convert_to_ascii: bool,
    ) -> Result<String> {
        let device_desc = handle.device().device_descriptor()?;
        let lang = handle.read_languages(DEFAULT_TIMEOUT)?;

        if convert_to_ascii {
            return Ok(handle.read_product_string_ascii(&device_desc)?);
        }

        Ok(handle.read_product_string(lang[0], &device_desc, DEFAULT_TIMEOUT)?)
    }

    /// Gets the product string in the current device.
    pub fn cp210x_rt_get_device_serial_string(
        &self,
        handle: &DeviceHandle<Context>,
        convert_to_ascii: bool,
    ) -> Result<String> {
        let device_desc = handle.device().device_descriptor()?;
        let lang = handle.read_languages(DEFAULT_TIMEOUT)?;

        if convert_to_ascii {
            return Ok(handle.read_serial_number_string_ascii(&device_desc)?);
        }

        Ok(handle.read_serial_number_string(lang[0], &device_desc, DEFAULT_TIMEOUT)?)
    }
}
