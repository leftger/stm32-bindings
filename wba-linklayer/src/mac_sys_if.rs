use core::mem::MaybeUninit;

use crate::ffi;

/// Handle identifying a MAC controller instance within the ST middleware.
///
/// The underlying value is the 32‑bit handle that the vendor functions expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MacHandle(u32);

impl MacHandle {
    /// Creates a new wrapper around the raw handle value.
    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw handle value expected by the FFI.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Result type used by the safe MAC wrappers.
pub type MacResult<T = ()> = Result<T, MacStatus>;

/// Error codes surfaced by the MAC middleware.
///
/// The enumeration focuses on the most frequently observed statuses. Any value
/// that is not explicitly handled is reported via [`MacStatus::Other`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacStatus {
    Success,
    Busy,
    InvalidParameter,
    ChannelAccessFailure,
    NoAck,
    Unsupported,
    InternalError,
    Other(u8),
}

impl MacStatus {
    #[inline]
    fn from_raw(value: ffi::mac_status_enum_t) -> Self {
        match value as u32 {
            ffi::MAC_STATUS_ENUM_T_MAC_STATUS_SUCCESS => Self::Success,
            ffi::MAC_STATUS_ENUM_T_DENIED => Self::Busy,
            ffi::MAC_STATUS_ENUM_T_INVALID_PARAMETER => Self::InvalidParameter,
            ffi::MAC_STATUS_ENUM_T_CHANNEL_ACCESS_FAILURE => Self::ChannelAccessFailure,
            ffi::MAC_STATUS_ENUM_T_NO_ACK => Self::NoAck,
            ffi::MAC_STATUS_ENUM_T_UNSUPPORTED_ATTRIBUTE
            | ffi::MAC_STATUS_ENUM_T_UNSUPPORTED_LEGACY
            | ffi::MAC_STATUS_ENUM_T_UNSUPPORTED_SECURITY => Self::Unsupported,
            ffi::MAC_STATUS_ENUM_T_INTERNAL_ERROR => Self::InternalError,
            other => Self::Other(other as u8),
        }
    }

    #[inline]
    fn into_result(self) -> MacResult {
        match self {
            Self::Success => Ok(()),
            err => Err(err),
        }
    }
}

/// Describes how often antenna diversity should rotate between antennas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntennaIntervalType {
    None,
    FixedTime,
    PacketCount,
}

impl AntennaIntervalType {
    fn to_raw(self) -> ffi::ant_intrv_type_enum_t {
        match self {
            Self::None => ffi::ANT_INTRV_TYPE_ENUM_NO_TYPE,
            Self::FixedTime => ffi::ANT_INTRV_TYPE_ENUM_FIXED_TIME,
            Self::PacketCount => ffi::ANT_INTRV_TYPE_ENUM_PACKETS_NUMBER,
        }
    }

    fn from_raw(raw: ffi::ant_intrv_type_enum_t) -> Self {
        match raw {
            ffi::ANT_INTRV_TYPE_ENUM_FIXED_TIME => Self::FixedTime,
            ffi::ANT_INTRV_TYPE_ENUM_PACKETS_NUMBER => Self::PacketCount,
            _ => Self::None,
        }
    }
}

/// Parameters passed to the vendor antenna-diversity configuration routines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AntennaDiversityConfig {
    pub interval_type: AntennaIntervalType,
    pub interval_value: u32,
    pub wanted_coord_short_address: u16,
    pub wanted_coord_extended_address: [u8; 8],
    pub max_rx_ack_retries: u8,
}

impl Default for AntennaDiversityConfig {
    fn default() -> Self {
        Self {
            interval_type: AntennaIntervalType::None,
            interval_value: 0,
            wanted_coord_short_address: 0,
            wanted_coord_extended_address: [0; 8],
            max_rx_ack_retries: 0,
        }
    }
}

impl AntennaDiversityConfig {
    fn to_raw(self) -> ffi::antenna_diversity_st {
        ffi::antenna_diversity_st {
            ant_intrv_type: self.interval_type.to_raw(),
            ant_intrv_value: self.interval_value,
            wntd_coord_shrt_addr: self.wanted_coord_short_address,
            wntd_coord_ext_addr: self.wanted_coord_extended_address,
            max_rx_ack_retries: self.max_rx_ack_retries,
        }
    }

    fn from_raw(raw: ffi::antenna_diversity_st) -> Self {
        Self {
            interval_type: AntennaIntervalType::from_raw(raw.ant_intrv_type),
            interval_value: raw.ant_intrv_value,
            wanted_coord_short_address: raw.wntd_coord_shrt_addr,
            wanted_coord_extended_address: raw.wntd_coord_ext_addr,
            max_rx_ack_retries: raw.max_rx_ack_retries,
        }
    }
}

/// Runtime configuration flags stored inside the vendor library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigLibraryParams {
    pub mac_layer_enabled: bool,
    pub openthread_1_2_support: bool,
    pub ack_all_frames_with_ar_bit: bool,
}

impl Default for ConfigLibraryParams {
    fn default() -> Self {
        Self {
            mac_layer_enabled: true,
            openthread_1_2_support: true,
            ack_all_frames_with_ar_bit: false,
        }
    }
}

impl ConfigLibraryParams {
    fn to_raw(self) -> ffi::config_lib_st {
        ffi::config_lib_st {
            mac_layer_build: u8::from(self.mac_layer_enabled),
            support_openthread_1_2: u8::from(self.openthread_1_2_support),
            ack_all_received_frames_with_ar_bit_set: u8::from(self.ack_all_frames_with_ar_bit),
        }
    }

    fn from_raw(raw: ffi::config_lib_st) -> Self {
        Self {
            mac_layer_enabled: raw.mac_layer_build != 0,
            openthread_1_2_support: raw.support_openthread_1_2 != 0,
            ack_all_frames_with_ar_bit: raw.ack_all_received_frames_with_ar_bit_set != 0,
        }
    }
}

/// Enable or disable CSMA globally.
#[inline]
pub fn set_csma_enabled(enabled: bool) {
    unsafe { ffi::mac_set_csma_en(u8::from(enabled)) };
}

/// Enable or disable Clear Channel Assessment globally.
#[inline]
pub fn set_cca_enabled(enabled: bool) {
    unsafe { ffi::mac_set_cca_en(u8::from(enabled)) };
}

/// Configure the CCA threshold (dBm) for the selected MAC instance.
pub fn set_cca_threshold(handle: MacHandle, threshold_dbm: i8) -> MacResult {
    let status = unsafe { ffi::mac_set_cca_threshold(handle.raw(), threshold_dbm) };
    MacStatus::from_raw(status).into_result()
}

/// Retrieve the current CCA threshold (dBm).
pub fn get_cca_threshold(handle: MacHandle) -> MacResult<i8> {
    let mut value = 0i8;
    let status = unsafe { ffi::mac_get_cca_threshold(handle.raw(), &mut value) };
    match MacStatus::from_raw(status) {
        MacStatus::Success => Ok(value),
        err => Err(err),
    }
}

/// Request the MAC layer to perform a software reset.
///
/// When `set_default_pib` is `true`, all PIB attributes are restored to their
/// default values; otherwise only runtime state is cleared.
#[inline]
pub fn mlme_reset(handle: MacHandle, set_default_pib: bool) {
    unsafe { ffi::mlme_rst_req(handle.raw(), u8::from(set_default_pib)) };
}

/// Apply antenna diversity configuration parameters.
pub fn set_antenna_diversity(handle: MacHandle, params: &AntennaDiversityConfig) -> MacResult {
    let mut raw = params.to_raw();
    let status = unsafe { ffi::mac_set_ant_div_params(handle.raw(), &mut raw as *mut _) };
    MacStatus::from_raw(status).into_result()
}

/// Fetch the current antenna diversity configuration.
pub fn antenna_diversity(handle: MacHandle) -> AntennaDiversityConfig {
    let mut raw = MaybeUninit::<ffi::antenna_diversity_st>::uninit();
    unsafe { ffi::mac_get_ant_div_params(handle.raw(), raw.as_mut_ptr()) };
    AntennaDiversityConfig::from_raw(unsafe { raw.assume_init() })
}

/// Enable or disable antenna diversity logic.
pub fn set_antenna_diversity_enabled(handle: MacHandle, enabled: bool) -> MacResult {
    let status = unsafe { ffi::mac_set_ant_div_enable(handle.raw(), u8::from(enabled)) };
    MacStatus::from_raw(status).into_result()
}

/// Select the default antenna index used for transmission and reception.
pub fn set_default_antenna(handle: MacHandle, antenna_id: u8) -> MacResult {
    let status = unsafe { ffi::mac_set_default_ant_id(handle.raw(), antenna_id) };
    MacStatus::from_raw(status).into_result()
}

/// Adjust the RSSI threshold used when evaluating antenna diversity.
pub fn set_antenna_diversity_rssi_threshold(handle: MacHandle, threshold_dbm: i8) -> MacResult {
    let status = unsafe { ffi::mac_set_ant_div_rssi_threshold(handle.raw(), threshold_dbm) };
    MacStatus::from_raw(status).into_result()
}

/// Update the per-instance configurable library parameters.
pub fn set_config_library_params(handle: MacHandle, params: &ConfigLibraryParams) -> MacResult {
    let mut raw = params.to_raw();
    let status = unsafe { ffi::mac_set_config_lib_params(handle.raw(), &mut raw as *mut _) };
    MacStatus::from_raw(status).into_result()
}

/// Retrieve the per-instance configurable library parameters.
pub fn config_library_params(handle: MacHandle) -> ConfigLibraryParams {
    let mut raw = MaybeUninit::<ffi::config_lib_st>::uninit();
    unsafe { ffi::mac_get_config_lib_params(handle.raw(), raw.as_mut_ptr()) };
    ConfigLibraryParams::from_raw(unsafe { raw.assume_init() })
}

/// Read the global configuration shared by all MAC instances.
#[cfg(target_os = "none")]
pub fn shared_config_library_params() -> ConfigLibraryParams {
    unsafe { ConfigLibraryParams::from_raw(ffi::G_CONFIG_LIB_PARAMS) }
}

#[cfg(not(target_os = "none"))]
pub fn shared_config_library_params() -> ConfigLibraryParams {
    ConfigLibraryParams::default()
}

/// Overwrite the global configuration shared by all MAC instances.
///
/// # Safety
///
/// The vendor middleware does not provide synchronization. Callers must ensure
/// that concurrent accesses across cores/threads are serialized if required.
#[cfg(target_os = "none")]
pub unsafe fn set_shared_config_library_params(params: &ConfigLibraryParams) {
    unsafe {
        ffi::G_CONFIG_LIB_PARAMS = params.to_raw();
    }
}

#[cfg(not(target_os = "none"))]
pub unsafe fn set_shared_config_library_params(_params: &ConfigLibraryParams) {}

/// Configure the vendor RTL polling period (milliseconds).
#[inline]
pub fn set_rtl_polling_time(handle: MacHandle, period_ms: u8) {
    unsafe { ffi::mac_set_rtl_polling_time(handle.raw(), period_ms) };
}

/// Query the vendor RTL polling period (milliseconds).
#[inline]
pub fn rtl_polling_time(handle: MacHandle) -> u8 {
    unsafe { ffi::mac_get_rtl_polling_time(handle.raw()) }
}
