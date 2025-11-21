#![no_std]

mod generated {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    mod std {
        pub mod os {
            pub mod raw {
                pub type c_uint = u32;
            }
        }
    }

    // core::include!(core::concat!(core::env!("OUT_DIR"), "/bindings.rs"));
}
