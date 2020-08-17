//!
//!
//!
//! Bastion Executor is NUMA-aware SMP based Fault-tolerant Executor
//!
//! Bastion Executor is a highly-available, fault-tolerant, async communication
//! oriented executor. Bastion's main idea is supplying a fully async runtime
//! with fault-tolerance to work on heavy loads.
//!
//! Main differences between other executors are:
//! * Uses SMP based execution scheme to exploit cache affinity on multiple cores and execution is
//! equally distributed over the system resources, which means utilizing the all system.
//! * Uses NUMA-aware allocation for scheduler's queues and exploit locality on server workloads.
//! * Tailored for creating middleware and working with actor model like concurrency and distributed communication.
//!
//! **NOTE:** Bastion Executor is independent of it's framework implementation.
//! It uses [lightproc] to encapsulate and provide fault-tolerance to your future based workloads.
//! You can use your futures with [lightproc] to run your workloads on Bastion Executor without the need to have framework.
//!
//! [lightproc]: https://docs.rs/lightproc
//!

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/bastion-rs/bastion/master/img/bastion-logo.png"
)]
// Discarded lints
#![allow(clippy::if_same_then_else)]
// Force missing implementations
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![cfg_attr(
    any(feature = "numanji", feature = "allocator-suite"),
    feature(allocator_api)
)]
#![cfg_attr(
    any(feature = "numanji", feature = "allocator-suite"),
    feature(nonnull_slice_from_raw_parts)
)]
#[macro_use]
mod macros;

pub mod allocator;
pub mod blocking;
pub mod load_balancer;
pub mod placement;
pub mod pool;
pub mod run;
pub mod run_queue;
pub mod sleepers;
pub mod worker;

///
/// Prelude of Bastion Executor
pub mod prelude {
    pub use crate::blocking::*;
    pub use crate::pool::*;
    pub use crate::run::*;
}
