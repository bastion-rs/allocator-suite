// This file is part of context-allocator. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT. No part of context-allocator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of context-allocator. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/context-allocator/master/COPYRIGHT.

use super::*;

pub mod current_allocator_in_use;
pub mod global_thread_and_coroutine_switchable_allocator;
pub mod local_allocator;
pub mod memory_range;
pub mod per_thread_state;
pub mod switchable_allocator;

pub mod prelude {
    pub use super::current_allocator_in_use::*;
    pub use super::global_thread_and_coroutine_switchable_allocator::*;
    pub use super::local_allocator::*;
    pub use super::memory_range::*;
    pub use super::per_thread_state::*;
    pub use super::switchable_allocator::*;
}
