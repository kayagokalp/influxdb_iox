#![deny(rustdoc::broken_intra_doc_links, rustdoc::bare_urls, rust_2018_idioms)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::explicit_iter_loop,
    clippy::future_not_send,
    clippy::use_self,
    clippy::clone_on_ref_ptr
)]

pub(crate) mod codec;
pub mod config;
pub mod core;
pub mod file;

#[cfg(feature = "kafka")]
pub mod kafka;

pub mod mock;

pub mod rskafka;

#[cfg(all(test, feature = "kafka"))]
pub mod rskafka_kafka_test;
