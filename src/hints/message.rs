//! `Hints` allow you to pass extra information to the server.
//!
//! Many of these are standardized by either:
//!
//! [galago-project spec](http://www.galago-project.org/specs/notification/0.9/x344.html) or
//! [gnome notification-spec](https://developer.gnome.org/notification-spec/#hints)
//!
//! Which of these are actually implemented depends strongly on the Notification server you talk to.
//! Usually the `get_capabilities()` gives some clues, but the standards usually mention much more
//! than is actually available.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, unused_imports)]


use super::{Hint, constants::*};
use crate ::Urgency;

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use crate::image::*;

use std::collections::{HashMap, HashSet};

/// All currently implemented `Hints` that can be sent.
///
/// as found on <https://developer.gnome.org/notification-spec/>
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct HintMessage(Hint);

impl From<Hint> for HintMessage {
    fn from(hint: Hint) -> Self {
        HintMessage(hint)
    }
}

impl std::ops::Deref for HintMessage {
    type Target = Hint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
