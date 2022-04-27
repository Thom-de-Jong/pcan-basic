//!
//!
//!

use crate::bus::UsbBus;
use crate::can::{Baudrate, HasCanRead, HasCanReadFd, HasCanWrite, HasCanWriteFd, Socket};
use crate::channel::Channel;
use crate::error::{PcanError, PcanOkError};
use crate::pcan;
use crate::special;
use crate::special::{
    HasBusOffAutoreset, HasFiveVoltsPower, HasInterframeDelay, HasListenOnly,
    HasSetBusOffAutoreset, HasSetFiveVoltsPower, HasSetInterframeDelay, HasSetListenOnly,
};
use std::ffi::c_void;

#[derive(Debug, PartialEq)]
pub struct UsbCanSocket {
    handle: u16,
}

impl UsbCanSocket {
    pub fn open(bus: UsbBus, baud: Baudrate) -> Result<UsbCanSocket, PcanError> {
        let handle = bus.into();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(UsbCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Drop trait implementation */

impl Drop for UsbCanSocket {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.handle) };
    }
}

/* Socket trait implementation */

impl Socket for UsbCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* Channel trait implementation */

impl Channel for UsbCanSocket {
    fn channel(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementation */

impl HasCanRead for UsbCanSocket {}

/* HasCanReadFd trait implementation */

impl HasCanReadFd for UsbCanSocket {}

/* HasCanWrite trait implementation */

impl HasCanWrite for UsbCanSocket {}

/* HasCanWriteFd trait implementation */

impl HasCanWriteFd for UsbCanSocket {}

/* SPECIAL BEHAVIOR */

/* FiveVoltsPower trait implementation */

impl HasFiveVoltsPower for UsbCanSocket {}
impl HasSetFiveVoltsPower for UsbCanSocket {}

/* BusOffAutoreset trait implementation */

impl HasBusOffAutoreset for UsbCanSocket {}
impl HasSetBusOffAutoreset for UsbCanSocket {}

/* ListenOnly trait implementation */

impl HasListenOnly for UsbCanSocket {}
impl HasSetListenOnly for UsbCanSocket {}

/* InterframeDelay trait implementation */

impl HasInterframeDelay for UsbCanSocket {}
impl HasSetInterframeDelay for UsbCanSocket {}
