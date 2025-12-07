//! Safe wrappers around a subset of the BLE system interface exposed by the
//! STM32 WBA wireless stack.
//!
//! The goal of this module is to minimise the amount of ad-hoc `unsafe` code
//! required by higher layers while still providing zero-cost access to the
//! underlying C APIs.  Only a carefully-curated portion of the enormous BLE
//! surface is exposed here; additional helpers can be added incrementally as
//! the need arises.

use core::marker::PhantomData;
use core::ptr::{self, NonNull};
use core::slice;

use crate::ffi;

/// Result type returned by helpers that wrap `ble_stat_t`-style status values.
pub type BleResult<T = ()> = Result<T, BleStatus>;

/// Commonly observed BLE controller status codes.
///
/// The values are derived from the constants defined in `ble_defs.h`.  Any
/// status that is not modelled explicitly will be surfaced through
/// [`BleStatus::Other`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BleStatus {
    Success,
    Busy,
    Pending,
    InvalidParameters,
    InsufficientResources,
    OutOfMemory,
    Timeout,
    Other(u8),
}

impl BleStatus {
    /// Convert a raw `ble_stat_t` (as returned by the vendor APIs) into the
    /// strongly-typed status representation.
    #[inline]
    pub fn from_raw(value: ffi::ble_stat_t) -> Self {
        match (value & 0xFF) as u8 {
            0x00 => Self::Success,
            0x93 => Self::Busy,
            0x95 => Self::Pending,
            0x92 => Self::InvalidParameters,
            0x64 => Self::InsufficientResources,
            0x98 => Self::OutOfMemory,
            0xFF => Self::Timeout,
            other => Self::Other(other),
        }
    }

    /// Returns the raw 8-bit status code.
    #[inline]
    pub const fn code(self) -> u8 {
        match self {
            Self::Success => 0x00,
            Self::Busy => 0x93,
            Self::Pending => 0x95,
            Self::InvalidParameters => 0x92,
            Self::InsufficientResources => 0x64,
            Self::OutOfMemory => 0x98,
            Self::Timeout => 0xFF,
            Self::Other(code) => code,
        }
    }

    /// Converts the status into a `Result`, treating [`BleStatus::Success`] as
    /// the success case.
    #[inline]
    pub fn into_result(self) -> BleResult<()> {
        match self {
            Self::Success => Ok(()),
            other => Err(other),
        }
    }
}

/// Callback used by the controller to hand HCI buffers back to the host
/// transport.
pub type HostCallback = unsafe extern "C" fn(*mut ffi::ble_buff_hdr_t) -> u8;

/// Callback invoked when the controller fails to enqueue a buffer because the
/// host queue is full.
pub type HostQueueFullCallback = unsafe extern "C" fn(*mut ffi::ble_buff_hdr_t);

/// Callback signature used for vendor-specific HCI command handling.
pub type ExternalCustomCallback = unsafe extern "C" fn(
    ocf: u16,
    packet: *mut u8,
    event_packet: *mut u8,
    params_length: *mut u8,
    return_command_type: *mut ffi::hci_return_command_type,
) -> ffi::ble_stat_t;

pub type DispatchTable = ffi::hci_dispatch_tbl;

/// RAII wrapper around the vendor-provided `ble_buff_hdr_t` buffers.
///
/// Buffers are automatically returned to the controller when dropped.
pub struct BleBuffer {
    ptr: NonNull<ffi::ble_buff_hdr_t>,
    _not_send_sync: PhantomData<core::cell::Cell<ffi::ble_buff_hdr_t>>,
}

impl BleBuffer {
    /// Attempts to allocate a fresh buffer from the controller.
    pub fn allocate() -> Option<Self> {
        let ptr = unsafe { ffi::hci_alloc_msg() };
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _not_send_sync: PhantomData,
        })
    }

    /// Wrap an existing raw pointer returned by the vendor middleware.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` is either null (in which case `None`
    /// is returned) or a valid buffer obtained from the controller and that it
    /// remains valid for the lifetime of the returned wrapper.
    pub unsafe fn from_raw(ptr: *mut ffi::ble_buff_hdr_t) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            _not_send_sync: PhantomData,
        })
    }

    #[inline]
    fn header(&self) -> &ffi::ble_buff_hdr_t {
        unsafe { self.ptr.as_ref() }
    }

    #[inline]
    fn header_mut(&mut self) -> &mut ffi::ble_buff_hdr_t {
        unsafe { self.ptr.as_mut() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.header().data_size as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.header().total_len as usize
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.header().data_offset as usize
    }

    #[inline]
    pub fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity());
        debug_assert!(len <= u16::MAX as usize);
        self.header_mut().data_size = len as u16;
    }

    #[inline]
    pub fn set_offset(&mut self, offset: usize) {
        debug_assert!(offset <= self.capacity());
        debug_assert!(offset <= u16::MAX as usize);
        self.header_mut().data_offset = offset as u16;
    }

    fn payload_parts(&self) -> Option<(NonNull<u8>, usize)> {
        let header = self.header();
        let len = header.data_size as usize;
        if len == 0 {
            return None;
        }
        let base = NonNull::new(header.buff_start)?;
        let offset = header.data_offset as usize;
        let end = offset.checked_add(len)?;
        if header.total_len != 0 {
            debug_assert!(end <= header.total_len as usize);
        }
        let ptr = unsafe { base.as_ptr().add(offset) };
        NonNull::new(ptr).map(|ptr| (ptr, len))
    }

    /// Borrow the packet payload as an immutable slice.
    #[inline]
    pub fn payload(&self) -> Option<&[u8]> {
        self.payload_parts()
            .map(|(ptr, len)| unsafe { slice::from_raw_parts(ptr.as_ptr() as *const u8, len) })
    }

    /// Borrow the packet payload as a mutable slice.
    #[inline]
    pub fn payload_mut(&mut self) -> Option<&mut [u8]> {
        self.payload_parts()
            .map(|(ptr, len)| unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), len) })
    }

    /// Overwrite the payload with `data`, adjusting length/offset as needed.
    ///
    /// Returns `Err(())` when the payload does not fit in the buffer.
    pub fn write_payload(&mut self, data: &[u8]) -> Result<(), ()> {
        if data.len() > self.capacity() {
            return Err(());
        }

        self.set_offset(0);
        self.set_len(data.len());

        match self.payload_mut() {
            Some(slot) => {
                slot.copy_from_slice(data);
                Ok(())
            }
            None => {
                if data.is_empty() {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }

    /// Returns the raw pointer without relinquishing ownership.
    #[inline]
    pub fn as_ptr(&self) -> *const ffi::ble_buff_hdr_t {
        self.ptr.as_ptr()
    }

    /// Returns the raw mutable pointer without relinquishing ownership.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut ffi::ble_buff_hdr_t {
        self.ptr.as_ptr()
    }

    /// Consumes the wrapper and returns the raw pointer, preventing the
    /// destructor from freeing it.
    #[inline]
    pub fn into_raw(self) -> *mut ffi::ble_buff_hdr_t {
        let ptr = self.ptr.as_ptr();
        core::mem::forget(self);
        ptr
    }
}

impl Drop for BleBuffer {
    fn drop(&mut self) {
        unsafe { ffi::hci_free_msg(self.ptr.as_ptr()) };
    }
}

/// Initialise the BLE controller glue logic.
///
/// # Safety
///
/// The supplied callback must adhere to the constraints expected by the
/// controller (no unwinding across the FFI boundary, ISR safety where
/// applicable, etc.).
pub unsafe fn init_controller(callback: HostCallback) {
    unsafe {
        ffi::ll_sys_ble_cntrl_init(Some(callback));
    }
}

/// Initialise the controller <-> host transport layer.
pub fn init_transport(callback: HostCallback) -> BleResult {
    let status = unsafe { ffi::ll_hci_init(Some(callback)) };
    BleStatus::from_raw(status).into_result()
}

pub fn init_with_dispatch_table(table: &DispatchTable) -> BleResult {
    let status = unsafe { ffi::ll_intf_init(table as *const _) };
    BleStatus::from_raw(status).into_result()
}

pub fn init_default_interface() -> BleResult {
    match dispatch_table() {
        Some(table) => init_with_dispatch_table(table),
        None => Err(BleStatus::InsufficientResources),
    }
}

pub fn dispatch_table() -> Option<&'static DispatchTable> {
    let mut table: *const DispatchTable = ptr::null();
    unsafe {
        ffi::hci_get_dis_tbl(&mut table);
    }
    if table.is_null() {
        None
    } else {
        Some(unsafe { &*table })
    }
}

pub fn reset_interface() -> BleResult {
    let status = unsafe { ffi::ll_intf_reset() };
    BleStatus::from_raw(status).into_result()
}

pub fn reset_system() {
    unsafe { ffi::ll_sys_reset() };
}

/// Register an optional callback that is invoked when the host queue is full.
pub unsafe fn register_queue_full_callback(callback: Option<HostQueueFullCallback>) {
    unsafe {
        ffi::hci_rgstr_hst_cbk_ll_queue_full(callback);
    }
}

/// Register the primary host callback that receives HCI buffers.
pub unsafe fn register_host_callback(callback: Option<HostCallback>) {
    unsafe {
        ffi::hci_rgstr_hst_cbk(callback);
    }
}

/// Register or clear the vendor-specific HCI command handler.
pub unsafe fn register_external_custom_callback(callback: Option<ExternalCustomCallback>) -> bool {
    unsafe { ffi::hci_rgstr_ble_external_custom_cbk(callback) != 0 }
}

/// Prepare the vendor event queues. This must be called once before invoking
/// any of the queue-related helpers.
pub fn init_event_queues() {
    unsafe { ffi::hci_init_events_queues() };
}

/// Enqueue a packet for delivery to the host.
pub fn queue_send_packet(buffer: BleBuffer) -> BleResult {
    let ptr = buffer.into_raw();
    let status = BleStatus::from_raw(unsafe { ffi::hci_queue_send_pckt(ptr) as ffi::ble_stat_t });
    if status == BleStatus::Success {
        Ok(())
    } else {
        unsafe { ffi::hci_free_msg(ptr) };
        Err(status)
    }
}

/// Helper that allocates a buffer and enqueues it after copying `payload` into it.
///
/// Returns [`BleStatus::InsufficientResources`] if a buffer cannot be obtained or
/// [`BleStatus::InvalidParameters`] when the payload exceeds the buffer capacity.
pub fn queue_packet(payload: &[u8]) -> BleResult {
    let mut buffer = BleBuffer::allocate().ok_or(BleStatus::InsufficientResources)?;
    buffer
        .write_payload(payload)
        .map_err(|_| BleStatus::InvalidParameters)?;
    queue_send_packet(buffer)
}

/// Update the LE event mask used by the controller.
pub fn set_le_event_mask(mask: &mut [u8; 8]) {
    unsafe { ffi::hci_ll_set_le_event_mask(mask.as_mut_ptr()) };
}

/// Update the classic/Bluetooth event mask used by the controller.
pub fn set_event_mask(mask: &mut [u8; 8]) {
    unsafe { ffi::hci_ll_set_event_mask(mask.as_mut_ptr()) };
}

/// Update the page 2 event mask used by the controller.
pub fn set_event_mask_page2(mask: &mut [u8; 8]) {
    unsafe { ffi::hci_ll_set_event_mask_page2(mask.as_mut_ptr()) };
}

/// Update the custom event mask used by the controller.
pub fn set_custom_event_mask(mask: u8) {
    unsafe { ffi::hci_ll_set_custom_event_mask(mask) };
}
