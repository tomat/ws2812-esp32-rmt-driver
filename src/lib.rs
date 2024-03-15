#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod driver;

pub use driver::{Ws2812Esp32RmtDriver, Ws2812Esp32RmtDriverError};

pub mod lib_embedded_graphics;
pub mod lib_smart_leds;

pub use lib_smart_leds::{LedPixelEsp32Rmt, Ws2812Esp32Rmt, RGBW8};
pub use smart_leds_trait::RGB8;
