pub const __WORDSIZE: u32 = 32;
pub const __HAS_SAFE_BUFFERS: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __HAS_PTRCHECK: u32 = 0;
pub const __HAS_BOUNDS_SAFETY_ATTRIBUTES: u32 = 0;
pub const USE_CLANG_TYPES: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 4088;
pub const __PTHREAD_ATTR_SIZE__: u32 = 36;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 40;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 4;
pub const __PTHREAD_COND_SIZE__: u32 = 24;
pub const __PTHREAD_ONCE_SIZE__: u32 = 4;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 124;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 12;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u32 = 2147483647;
pub const INTPTR_MIN: i32 = -2147483648;
pub const UINTPTR_MAX: u32 = 4294967295;
pub const PTRDIFF_MIN: i32 = -2147483648;
pub const PTRDIFF_MAX: u32 = 2147483647;
pub const SIZE_MAX: u32 = 4294967295;
pub const RSIZE_MAX: u32 = 2147483647;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const ADV_IND: u32 = 0;
pub const ADV_DIRECT_IND: u32 = 1;
pub const ADV_SCAN_IND: u32 = 2;
pub const ADV_NONCONN_IND: u32 = 3;
pub const SCAN_RSP: u32 = 4;
pub const HIGH_DUTY_CYCLE_DIRECTED_ADV: u32 = 1;
pub const LOW_DUTY_CYCLE_DIRECTED_ADV: u32 = 4;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::core::ffi::c_schar;
pub type __uint8_t = ::core::ffi::c_uchar;
pub type __int16_t = ::core::ffi::c_short;
pub type __uint16_t = ::core::ffi::c_ushort;
pub type __int32_t = ::core::ffi::c_int;
pub type __uint32_t = ::core::ffi::c_uint;
pub type __int64_t = ::core::ffi::c_longlong;
pub type __uint64_t = ::core::ffi::c_ulonglong;
pub type __darwin_intptr_t = ::core::ffi::c_long;
pub type __darwin_natural_t = ::core::ffi::c_uint;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::core::ffi::c_char; 128usize],
    pub _mbstateL: ::core::ffi::c_longlong,
}
impl Default for __mbstate_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::core::ffi::c_int;
pub type __darwin_size_t = ::core::ffi::c_uint;
pub type __darwin_va_list = u32;
pub type __darwin_wchar_t = ::core::ffi::c_uint;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::core::ffi::c_int;
pub type __darwin_clock_t = ::core::ffi::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::core::ffi::c_long;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::core::ffi::c_uint;
pub type __darwin_fsfilcnt_t = ::core::ffi::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::core::ffi::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::core::ffi::c_char; 37usize];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
    pub __arg: *mut ::core::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
impl Default for __darwin_pthread_handler_rec {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 36usize],
}
impl Default for _opaque_pthread_attr_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 24usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 40usize],
}
impl Default for _opaque_pthread_mutex_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 8usize],
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 124usize],
}
impl Default for _opaque_pthread_rwlock_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::core::ffi::c_long,
    pub __opaque: [::core::ffi::c_char; 12usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::core::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::core::ffi::c_char; 4088usize],
}
impl Default for _opaque_pthread_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::core::ffi::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type intmax_t = ::core::ffi::c_longlong;
pub type uintmax_t = ::core::ffi::c_ulonglong;
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32 {
    pub v: u32,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT16_WRITE {
    pub v: u16,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT16_READ {
    pub v: u16,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32_WRITE {
    pub v: u32,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct T_UINT32_READ {
    pub v: u32,
}
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct _hci_uart_pckt {
    pub type_: u8,
    pub data: [u8; 1usize],
}
pub type hci_uart_pckt = _hci_uart_pckt;
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct _hci_event_pckt {
    pub evt: u8,
    pub plen: u8,
    pub data: [u8; 1usize],
}
pub type hci_event_pckt = _hci_event_pckt;
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct _evt_le_meta_event {
    pub subevent: u8,
    pub data: [u8; 1usize],
}
pub type evt_le_meta_event = _evt_le_meta_event;
#[doc = " Vendor specific event for BLE core."]
#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
pub struct _evt_blecore_aci {
    #[doc = "< One of the BLE core event codes."]
    pub ecode: u16,
    pub data: [u8; 1usize],
}
#[doc = " Vendor specific event for BLE core."]
pub type evt_blecore_aci = _evt_blecore_aci;
pub const SVCCTL_EvtAckStatus_t_SVCCTL_EvtNotAck: SVCCTL_EvtAckStatus_t = 0;
pub const SVCCTL_EvtAckStatus_t_SVCCTL_EvtAckFlowEnable: SVCCTL_EvtAckStatus_t = 1;
pub const SVCCTL_EvtAckStatus_t_SVCCTL_EvtAckFlowDisable: SVCCTL_EvtAckStatus_t = 2;
pub type SVCCTL_EvtAckStatus_t = ::core::ffi::c_uint;
pub const SVCCTL_UserEvtFlowStatus_t_SVCCTL_UserEvtFlowDisable: SVCCTL_UserEvtFlowStatus_t = 0;
pub const SVCCTL_UserEvtFlowStatus_t_SVCCTL_UserEvtFlowEnable: SVCCTL_UserEvtFlowStatus_t = 1;
pub type SVCCTL_UserEvtFlowStatus_t = ::core::ffi::c_uint;
pub type SVC_CTL_p_EvtHandler_t = ::core::option::Option<
    unsafe extern "C" fn(p_evt: *mut ::core::ffi::c_void) -> SVCCTL_EvtAckStatus_t,
>;
unsafe extern "C" {
    #[doc = " @brief  It initializes the BLE core Driver and sends some commands to initialize the BLE core device\n         It shall be called before any BLE operation\n\n @param  None\n @retval None"]
    pub fn SVCCTL_Init();
}
unsafe extern "C" {
    #[doc = " @brief  This API registers a handler to be called when a GATT user event is received from the BLE core device. When\n         a Service is created, it shall register a callback to be notified when a GATT event is received from the\n         BLE core device. When a GATT event is received, it shall be checked in the handler if the GATT events belongs\n         to the Service or not. The handler shall return the correct status depending on the result. As soon as one\n         Service handler registered acknowledges positively the GATT event, the ble_controller stops calling the\n         registered Service handlers.\n         This handler is called in the TL_BLE_HCI_UserEvtProc() context\n\n @param  pfBLE_SVC_Service_Event_Handler: This is the Service handler that the ble_controller calls to report a GATT\n         event received. If the GATT event belongs to that Service, the callback shall return positively with\n         SVCCTL_EvtAckFlowEnable.\n @retval None"]
    pub fn SVCCTL_RegisterSvcHandler(pfBLE_SVC_Service_Event_Handler: SVC_CTL_p_EvtHandler_t);
}
unsafe extern "C" {
    #[doc = " @brief  This API registers a handler to be called when a GATT user event is received from the BLE core device. When\n         a Client is created, it shall register a callback to be notified when a GATT event is received from the\n         BLE core device. When a GATT event is received, it shall be checked in the handler if the GATT events belongs\n         to the Client or not. The handler shall return the correct status depending on the result. As soon as one\n         Client handler registered acknowledges positively the GATT event, the ble_controller stops calling the\n         registered Client handlers.\n         This handler is called in the TL_BLE_HCI_UserEvtProc() context\n\n @param  pfBLE_SVC_Client_Event_Handler: This is the Client handler that the ble_controller calls to report a GATT\n         event received. If the GATT event belongs to that Client, the callback shall return positively with\n         SVCCTL_EvtAckFlowEnable.\n @retval None"]
    pub fn SVCCTL_RegisterCltHandler(pfBLE_SVC_Client_Event_Handler: SVC_CTL_p_EvtHandler_t);
}
unsafe extern "C" {
    #[doc = " @brief  This API registers a handler to be called when a GAP user event is received from the BLE core device. When\n         a Profile is created, it could register a callback to be notified when a GAP event is received from the\n         BLE core device.\n         As soon as one handler registered acknowledges positively the event, the ble_controller stops calling the\n         registered handlers.\n         This handler is called in the TL_BLE_HCI_UserEvtProc() context\n\n @param  pfBLE_SVC_Event_Handler: This is the  handler that the ble_controller calls to report a GAP\n         event received.\n @retval None"]
    pub fn SVCCTL_RegisterHandler(pfBLE_SVC_Event_Handler: SVC_CTL_p_EvtHandler_t);
}
unsafe extern "C" {
    #[doc = " @brief  This API is used to resume the User Event Flow that has been stopped in return of SVCCTL_UserEvtRx()\n\n @param  None\n @retval None"]
    pub fn SVCCTL_ResumeUserEventFlow();
}
unsafe extern "C" {
    #[doc = " @brief This callback is triggered when either\n          + a GAP event is received from the BLE core device.\n          + a GATT event that has not been positively acknowledged by the registered handler is received from the\n            BLE core device.\n        The event is returned in a HCI packet. The full HCI packet is stored in a single buffer and is available when\n        this callback is triggered. However, an ACI event may be longer than a HCI packet and could be fragmented over\n        several HCI packets. The HCI layer only handles HCI packets so when an ACI packet is split over several HCI\n        packets, this callback is triggered for each HCI fragment. It is the responsibility of the application to\n        reassemble the ACI event.\n        This callback is triggered in the TL_BLE_HCI_UserEvtProc() context\n\n @param  pckt: The user event received from the BLE core device\n @retval None"]
    pub fn SVCCTL_App_Notification(pckt: *mut ::core::ffi::c_void) -> SVCCTL_UserEvtFlowStatus_t;
}
unsafe extern "C" {
    #[doc = " @brief\n\n\n @param  pckt: The user event received from the BLE core device\n @retval SVCCTL_UserEvtFlowStatus_t: SVCCTL_UserEvtFlowEnable when the packet has been processed\n         SVCCTL_UserEvtFlowDisable otherwise (the packet is kept in the queue)"]
    pub fn SVCCTL_UserEvtRx(pckt: *mut ::core::ffi::c_void) -> SVCCTL_UserEvtFlowStatus_t;
}
