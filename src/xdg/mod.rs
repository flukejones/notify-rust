//! This module contains `XDG` and `DBus` specific code.
//!
//! it should not be available under any platform other than `(unix, not(target_os = "macos"))`

use crate::{error::*, notification::Notification};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "zbus")]
mod zbus_rs;

#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_NAMESPACE: &str = "org.freedesktop.Notifications";
#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_OBJECTPATH: &str = "/org/freedesktop/Notifications";

#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_NAMESPACE: &str = "de.hoodie.Notifications";
#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_OBJECTPATH: &str = "/de/hoodie/Notifications";

#[derive(Debug)]
enum NotificationHandleInner {
    #[cfg(feature = "zbus")]
    Zbus(zbus_rs::ZbusNotificationHandle),
}

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    inner: NotificationHandleInner,
}

#[allow(dead_code)]
impl NotificationHandle {
    #[cfg(feature = "zbus")]
    pub(crate) fn for_zbus(
        id: u32,
        connection: zbus::blocking::Connection,
        notification: Notification,
    ) -> NotificationHandle {
        NotificationHandle {
            inner: zbus_rs::ZbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                inner
                    .wait_for_action(|action: &ActionResponse| match action {
                        ActionResponse::Custom(action) => invocation_closure(action),
                        ActionResponse::Closed(_reason) => invocation_closure("__closed"), // FIXME: remove backward compatibility with 5.0
                    })
                    .ok();
            } // TODO: pass error to caller
        };
    }

    /// Manually close the notification
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use notify_rust::*;
    /// let handle: NotificationHandle = Notification::new()
    ///     .summary("oh no")
    ///     .hint(notify_rust::Hint::Transient(true))
    ///     .body("I'll be here till you close me!")
    ///     .hint(Hint::Resident(true)) // does not work on kde
    ///     .timeout(Timeout::Never) // works on kde and gnome
    ///     .show()
    ///     .unwrap();
    /// // ... and then later
    /// handle.close();
    /// ```
    pub fn close(self) {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                // TODO: handle error
                inner.close().ok();
            }
        }
    }

    /// Executes a closure after the notification has closed.
    ///
    /// ## Example 1: *I don't care about why it closed* (the good ole API)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|| println!("closed"));
    /// ```
    ///
    /// ## Example 2: *I **do** care about why it closed* (added in v4.5.0)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|reason| println!("closed: {:?}", reason));
    /// ```
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                inner
                    .wait_for_action(|action: &ActionResponse| {
                        if let ActionResponse::Closed(reason) = action {
                            handler.call(*reason);
                        }
                    })
                    .ok();
            } // TODO: pass error to caller
        };
    }

    /// Replace the original notification with an updated version
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut notification = Notification::new().summary("Latest News")
    ///                                           .body("Bayern Dortmund 3:2")
    ///                                           .show()
    ///                                           .unwrap();
    ///
    /// std::thread::sleep_ms(1_500);
    ///
    /// notification.summary("Latest News (Correction)")
    ///             .body("Bayern Dortmund 3:3");
    ///
    /// notification.update();
    /// ```
    /// Watch out for different implementations of the
    /// notification server! On plasma5 for instance, you should also change the appname, so the old
    /// message is really replaced and not just amended. Xfce behaves well, all others have not
    /// been tested by the developer.
    pub fn update(&mut self) {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => {
                // TODO: handle error
                inner.update().ok();
            }
        }
    }

    /// Returns the Handle's id.
    pub fn id(&self) -> u32 {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => inner.id,
        }
    }
}

/// Required for `DerefMut`
impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => &inner.notification,
        }
    }
}

/// Allow you to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        match self.inner {
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => &mut inner.notification,
        }
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandleInner {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandleInner {
        NotificationHandleInner::Zbus(handle)
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandle {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandle {
        NotificationHandle {
            inner: handle.into(),
        }
    }
}

// here be public functions

#[cfg(feature = "zbus")]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    zbus_rs::connect_and_send_notification(notification).map(Into::into)
}

/// Get the currently dsed [`DbusStack`]
///
/// neither zbus nor dbus-rs are configured
#[cfg(not(feature = "zbus"))]
pub fn dbus_stack() -> Option<DbusStack> {
    None
}

/// Get list of all capabilities of the running notification server.
///
/// (zbus only)
#[cfg(feature = "zbus")]
pub fn get_capabilities() -> Result<Vec<String>> {
    zbus_rs::get_capabilities()
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
///
/// (zbus only)
#[cfg(feature = "zbus")]
pub fn get_server_information() -> Result<ServerInformation> {
    zbus_rs::get_server_information()
}

/// Return value of `get_server_information()`.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "zbus", derive(zbus::zvariant::Type))]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,
    /// The vendor name.
    pub vendor: String,
    /// The server's version string.
    pub version: String,
    /// The specification version the server is compliant with.
    pub spec_version: String,
}

/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() {}

/// Reased passed to `NotificationClosed` Signal
///
/// ## Specification
/// As listed under [Table 8. `NotificationClosed` Parameters](https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html#idm46350804042704)
#[derive(Copy, Clone, Debug)]
pub enum CloseReason {
    /// The notification expired
    Expired,
    /// The notification was dismissed by the user
    Dismissed,
    /// The notification was closed by a call to `CloseNotification`
    CloseAction,
    /// Undefined/Reserved reason
    Other(u32),
}

impl From<u32> for CloseReason {
    fn from(raw_reason: u32) -> Self {
        match raw_reason {
            1 => CloseReason::Expired,
            2 => CloseReason::Dismissed,
            3 => CloseReason::CloseAction,
            other => CloseReason::Other(other),
        }
    }
}

/// Helper Trait implemented by `Fn()`
pub trait ActionResponseHandler {
    fn call(self, response: &ActionResponse);
}

// impl<F: Send + Sync + 'static> ActionResponseHandler for F
impl<F> ActionResponseHandler for F
where
    F: FnOnce(&ActionResponse),
{
    fn call(self, res: &ActionResponse) {
        (self)(res);
    }
}

/// Response to an action
pub enum ActionResponse<'a> {
    /// Custom Action configured by the Notification.
    Custom(&'a str),

    /// The Notification was closed.
    Closed(CloseReason),
}

impl<'a> From<&'a str> for ActionResponse<'a> {
    fn from(raw: &'a str) -> Self {
        Self::Custom(raw)
    }
}

/// Your handy callback for the `Close` signal of your Notification.
///
/// This is implemented by `Fn()` and `Fn(CloseReason)`, so there is probably no good reason for you to manually implement this trait.
/// Should you find one anyway, please notify me and I'll gladly remove this obviously redundant comment.
pub trait CloseHandler<T> {
    /// This is called with the [`CloseReason`].
    fn call(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn call(&self, reason: CloseReason) {
        self(reason);
    }
}

impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn call(&self, _: CloseReason) {
        self();
    }
}
