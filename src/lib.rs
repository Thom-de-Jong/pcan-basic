//!
//!

pub mod bus;
pub mod error;

use crate::bus::{DngBus, IsaBus, LanBus, PciBus, ToHandle, UsbBus};
use crate::error::{PcanError, PcanOkError};
use pcan_basic_sys as pcan;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Standard,
    Extended,
}

#[derive(Debug, PartialEq)]
pub enum FrameConstructionError {
    TooMuchData,
    CanIdMessageTypeMismatch,
}

pub const STANDARD_MASK: u32 = 0x07_FF;
pub const EXTENDED_MASK: u32 = 0x1F_FF_FF_FF;

#[derive(Debug, Copy, Clone)]
pub struct CanFrame {
    frame: pcan::TPCANMsg,
}

impl CanFrame {
    const MAX_DLC: usize = 8;

    pub fn new(
        can_id: u32,
        msg_type: MessageType,
        data: &[u8],
    ) -> Result<CanFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 8] = [0; 8];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => Ok(CanFrame {
                    frame: pcan::TPCANMsg {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        LEN: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
                MessageType::Extended => Ok(CanFrame {
                    frame: pcan::TPCANMsg {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        LEN: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.LEN
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.LEN != other.frame.LEN {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CanFdFrame {
    frame: pcan::TPCANMsgFD,
}

impl CanFdFrame {
    const MAX_DLC: usize = 64;

    pub fn new(
        can_id: u32,
        msg_type: MessageType,
        data: &[u8],
    ) -> Result<CanFdFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 64] = [0; 64];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => Ok(CanFdFrame {
                    frame: pcan::TPCANMsgFD {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        DLC: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
                MessageType::Extended => Ok(CanFdFrame {
                    frame: pcan::TPCANMsgFD {
                        ID: can_id & STANDARD_MASK,
                        MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                        DLC: data.len() as u8,
                        DATA: frame_data,
                    },
                }),
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.DLC
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFdFrame {
    fn default() -> Self {
        CanFdFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFdFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.DLC != other.frame.DLC {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Timestamp {
    timestamp: pcan::TPCANTimestamp,
}

impl Default for Timestamp {
    fn default() -> Timestamp {
        Timestamp {
            timestamp: pcan::TPCANTimestamp {
                micros: 0,
                millis: 0,
                millis_overflow: 0,
            },
        }
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        if self.timestamp.micros != other.timestamp.micros {
            return false;
        }

        if self.timestamp.millis != other.timestamp.millis {
            return false;
        }

        if self.timestamp.millis_overflow != other.timestamp.millis_overflow {
            return false;
        }

        true
    }
}

pub trait CanRead {
    fn read(&self) -> Result<(CanFrame, Timestamp), PcanError>;
    fn read_frame(&self) -> Result<CanFrame, PcanError>;
}

pub trait CanReadFd {
    fn read(&self) -> Result<(CanFdFrame, u64), PcanError>;
    fn read_frame(&self) -> Result<CanFdFrame, PcanError>;
}

pub trait CanWrite {
    fn write(&self, frame: CanFrame) -> Result<(), PcanError>;
}

pub trait CanWriteFd {
    fn write(&self, frame: CanFdFrame) -> Result<(), PcanError>;
}

trait Socket {
    fn handle(&self) -> u16;
}

trait HasCanRead {}
trait HasCanReadFd {}
trait HasCanWrite {}
trait HasCanWriteFd {}

/* Baudrate */

#[derive(Debug, PartialEq)]
pub enum Baudrate {
    Baud1M,
    Baud800K,
    Baud500K,
    Baud250K,
    Baud125K,
    Baud100K,
    Baud95K,
    Baud83,
    Baud50K,
    Baud47K,
    Baud33K,
    Baud20K,
    Baud10K,
    Baud5K,
}

impl From<Baudrate> for u16 {
    fn from(value: Baudrate) -> Self {
        let ret = match value {
            Baudrate::Baud1M => pcan::PCAN_BAUD_1M,
            Baudrate::Baud800K => pcan::PCAN_BAUD_800K,
            Baudrate::Baud500K => pcan::PCAN_BAUD_500K,
            Baudrate::Baud250K => pcan::PCAN_BAUD_250K,
            Baudrate::Baud125K => pcan::PCAN_BAUD_125K,
            Baudrate::Baud100K => pcan::PCAN_BAUD_100K,
            Baudrate::Baud95K => pcan::PCAN_BAUD_95K,
            Baudrate::Baud83 => pcan::PCAN_BAUD_83K,
            Baudrate::Baud50K => pcan::PCAN_BAUD_50K,
            Baudrate::Baud47K => pcan::PCAN_BAUD_47K,
            Baudrate::Baud33K => pcan::PCAN_BAUD_33K,
            Baudrate::Baud20K => pcan::PCAN_BAUD_20K,
            Baudrate::Baud10K => pcan::PCAN_BAUD_10K,
            Baudrate::Baud5K => pcan::PCAN_BAUD_5K,
        } as u16;
        ret
    }
}

/* CAN socket types */

pub struct IsaCanSocket {
    handle: u16,
}

impl IsaCanSocket {
    pub fn new(bus: IsaBus, baud: Baudrate) -> Result<IsaCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(IsaCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct DngCanSocket {
    handle: u16,
}

impl DngCanSocket {
    pub fn new(bus: DngBus, baud: Baudrate) -> Result<DngCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(DngCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct PciCanSocket {
    handle: u16,
}

impl PciCanSocket {
    pub fn new(bus: PciBus, baud: Baudrate) -> Result<PciCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PciCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct PccCanSocket {
    handle: u16,
}

impl PccCanSocket {
    pub fn new(bus: PciBus, baud: Baudrate) -> Result<PccCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(PccCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct UsbCanSocket {
    handle: u16,
}

impl UsbCanSocket {
    pub fn new(bus: UsbBus, baud: Baudrate) -> Result<UsbCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(UsbCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct LanCanSocket {
    handle: u16,
}

impl LanCanSocket {
    pub fn new(bus: LanBus, baud: Baudrate) -> Result<LanCanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(LanCanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

pub struct CanSocket {
    handle: u16,
}

impl CanSocket {
    pub fn new<T: ToHandle>(bus: T, baud: Baudrate) -> Result<CanSocket, PcanError> {
        let handle = bus.handle();
        let code = unsafe { pcan::CAN_Initialize(handle, baud.into(), 0, 0, 0) };

        match PcanOkError::try_from(code) {
            Ok(PcanOkError::Ok) => Ok(CanSocket { handle }),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* Socket trait implementations */

impl Socket for IsaCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for DngCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for PciCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for PccCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for UsbCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for LanCanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

impl Socket for CanSocket {
    fn handle(&self) -> u16 {
        self.handle
    }
}

/* HasCanRead trait implementations */

impl HasCanRead for IsaCanSocket {}
impl HasCanRead for DngCanSocket {}
impl HasCanRead for PciCanSocket {}
impl HasCanRead for PccCanSocket {}
impl HasCanRead for UsbCanSocket {}
impl HasCanRead for LanCanSocket {}
impl HasCanRead for CanSocket {}

/* HasCanReadFd trait implementations */

impl HasCanReadFd for IsaCanSocket {}
impl HasCanReadFd for DngCanSocket {}
impl HasCanReadFd for PciCanSocket {}
impl HasCanReadFd for PccCanSocket {}
impl HasCanReadFd for UsbCanSocket {}
impl HasCanReadFd for LanCanSocket {}
impl HasCanReadFd for CanSocket {}

/* HasCanWrite trait implementations */

impl HasCanWrite for IsaCanSocket {}
impl HasCanWrite for DngCanSocket {}
impl HasCanWrite for PciCanSocket {}
impl HasCanWrite for PccCanSocket {}
impl HasCanWrite for UsbCanSocket {}
impl HasCanWrite for LanCanSocket {}
impl HasCanWrite for CanSocket {}

/* HasCanWriteFd trait implementations */

impl HasCanWriteFd for IsaCanSocket {}
impl HasCanWriteFd for DngCanSocket {}
impl HasCanWriteFd for PciCanSocket {}
impl HasCanWriteFd for PccCanSocket {}
impl HasCanWriteFd for UsbCanSocket {}
impl HasCanWriteFd for LanCanSocket {}
impl HasCanWriteFd for CanSocket {}

/* Drop trait implementations */

struct SocketDropWrapper<T: Socket> {
    socket: T,
}

impl<T: Socket> Drop for SocketDropWrapper<T> {
    fn drop(&mut self) {
        unsafe { pcan::CAN_Uninitialize(self.socket.handle()) };
    }
}

/* CanRead trait implementations */

impl<T: Socket + HasCanRead> CanRead for T {
    fn read(&self) -> Result<(CanFrame, Timestamp), PcanError> {
        let mut frame = CanFrame::default();
        let mut timestamp = Timestamp::default();

        let error_code = unsafe {
            pcan::CAN_Read(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsg,
                &mut timestamp.timestamp as *mut pcan::TPCANTimestamp,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok((frame, timestamp)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn read_frame(&self) -> Result<CanFrame, PcanError> {
        let mut frame = CanFrame::default();

        let error_code = unsafe {
            pcan::CAN_Read(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsg,
                0 as *mut pcan::TPCANTimestamp,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(frame),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanFdRead trait implementation */

impl<T: Socket + HasCanReadFd> CanReadFd for T {
    fn read(&self) -> Result<(CanFdFrame, u64), PcanError> {
        let mut frame = CanFdFrame::default();
        let mut timestamp = 0u64;

        let error_code = unsafe {
            pcan::CAN_ReadFD(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsgFD,
                &mut timestamp as *mut u64,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok((frame, timestamp)),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }

    fn read_frame(&self) -> Result<CanFdFrame, PcanError> {
        let mut frame = CanFdFrame::default();

        let error_code = unsafe {
            pcan::CAN_ReadFD(
                self.handle(),
                &mut frame.frame as *mut pcan::TPCANMsgFD,
                0 as *mut u64,
            )
        };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(frame),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanWrite trait implementations */

impl<T: Socket + HasCanWrite> CanWrite for T {
    fn write(&self, frame: CanFrame) -> Result<(), PcanError> {
        let mut frame = frame;
        let error_code =
            unsafe { pcan::CAN_Write(self.handle(), &mut frame.frame as *mut pcan::TPCANMsg) };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

/* CanWriteFd trait implementation */

impl<T: Socket + HasCanWriteFd> CanWriteFd for T {
    fn write(&self, frame: CanFdFrame) -> Result<(), PcanError> {
        let mut frame = frame;
        let error_code =
            unsafe { pcan::CAN_WriteFD(self.handle(), &mut frame.frame as *mut pcan::TPCANMsgFD) };

        match PcanOkError::try_from(error_code) {
            Ok(PcanOkError::Ok) => Ok(()),
            Ok(PcanOkError::Err(err)) => Err(err),
            Err(_) => Err(PcanError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_frame_new_001() {
        let can_frame_1 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let can_frame_2 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_frame_new_002() {
        let can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        let can_frame_2 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_frame_new_003() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Standard, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_frame_new_004() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    }

    /* CAN FD FRAME */

    #[test]
    fn can_fd_frame_new_001() {
        let can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();

        let can_frame_2 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..64u8).collect::<Vec<_>>()).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_fd_frame_new_002() {
        let can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();

        let can_frame_2 =
            CanFdFrame::new(0x20, MessageType::Extended, &(0..64u8).collect::<Vec<_>>()).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_003() {
        let _can_frame_1 =
            CanFdFrame::new(0x20, MessageType::Standard, &(0..65u8).collect::<Vec<_>>()).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_004() {
        let _can_frame_1 =
            CanFrame::new(0x20, MessageType::Extended, &(0..65u8).collect::<Vec<_>>()).unwrap();
    }
}
