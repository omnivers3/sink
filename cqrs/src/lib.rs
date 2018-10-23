#![feature(associated_type_defaults)]
// Support using this library without the standard library
#![cfg_attr(not(feature = "std"), no_std)]
// #[no_std]
////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "alloc")]
extern crate alloc;

/// A facade around all the types we need from the `std`, `core`, and `alloc`
/// crates. This avoids elaborate import wrangling having to happen in every
/// module.
mod lib {
    pub mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::{cmp, iter, mem, num, slice, str};
    pub use self::core::{f32, f64};
    pub use self::core::{i16, i32, i64, i8, isize};
    pub use self::core::{u16, u32, u64, u8, usize};

    pub use self::core::cell::{Cell, RefCell};
    pub use self::core::clone::{self, Clone};
    pub use self::core::convert::{self, From, Into};
    pub use self::core::default::{self, Default};
    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::marker::{self, PhantomData};
    pub use self::core::ops::Range;
    pub use self::core::option::{self, Option};
    pub use self::core::result::{self, Result};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::borrow::{Cow, ToOwned};
    #[cfg(feature = "std")]
    pub use std::borrow::{Cow, ToOwned};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(all(feature = "rc", feature = "alloc", not(feature = "std")))]
    pub use alloc::rc::{Rc, Weak as RcWeak};
    #[cfg(all(feature = "rc", feature = "std"))]
    pub use std::rc::{Rc, Weak as RcWeak};

    #[cfg(all(feature = "rc", feature = "alloc", not(feature = "std")))]
    pub use alloc::sync::{Arc, Weak as ArcWeak};
    #[cfg(all(feature = "rc", feature = "std"))]
    pub use std::sync::{Arc, Weak as ArcWeak};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
    #[cfg(feature = "std")]
    pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

    #[cfg(feature = "std")]
    pub use std::{error, net};

    #[cfg(feature = "std")]
    pub use std::collections::{HashMap, HashSet};
    #[cfg(feature = "std")]
    pub use std::ffi::{CStr, CString, OsStr, OsString};
    #[cfg(feature = "std")]
    pub use std::hash::{BuildHasher, Hash};
    #[cfg(feature = "std")]
    pub use std::io::Write;
    #[cfg(feature = "std")]
    pub use std::num::Wrapping;
    #[cfg(feature = "std")]
    pub use std::path::{Path, PathBuf};
    #[cfg(feature = "std")]
    pub use std::sync::{Mutex, RwLock};
    #[cfg(feature = "std")]
    pub use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(any(core_duration, feature = "std"))]
    pub use self::core::time::Duration;

    #[cfg(range_inclusive)]
    pub use self::core::ops::RangeInclusive;
}

extern crate sink;

// pub mod echo;
// pub mod map;
// pub mod reflect;

pub mod bounded_counter;
// pub mod iasyncsink;

// pub mod sink;

// pub mod echosink;
// pub mod statefulsink;
// pub mod vecsink;

// pub use self::iaggregate::*;
// pub use self::iasyncsink::*;
// pub use self::echo::*;
// pub use self::map::*;
// pub use self::reflect::*;

pub trait IAggregate {
    type TState;
    type TCommands;
    type TEvents;
    type TErrors;

    fn apply(&self, state: &mut Self::TState, event: Self::TEvents);
    fn eval(
        &self,
        state: Self::TState,
        command: Self::TCommands,
    ) -> Result<Self::TEvents, Self::TErrors>;
}

pub trait IAggregateMeta {
    fn domain() -> String;
    fn commands() -> Vec<String>;
    fn events() -> Vec<String>;
}

pub enum AggregateCommands<TIdentity, TVersion, TState, TCommands, TEvents> {
    HandleCommand(TIdentity, TVersion, TState, TCommands),
    LoadEvents(TIdentity),
    LoadState(TIdentity, Vec<TEvents>),
}

pub enum AggregateEvents<TIdentity, TVersion, TCommands, TEvents, TErrors> {
    DispatchEvent(TIdentity, TVersion, TEvents),
    DispatchError(TIdentity, TVersion, TCommands, TErrors),
}
