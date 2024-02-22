use crate::{error::*, notification::Notification, xdg};
use zbus::{blocking::Connection, MatchRule};

use super::{ActionResponse, ActionResponseHandler, CloseReason};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct ZbusNotificationHandle {
    pub(crate) id: u32,
    pub(crate) connection: Connection,
    pub(crate) notification: Notification,
}

impl ZbusNotificationHandle {
    pub(crate) fn new(
        id: u32,
        connection: Connection,
        notification: Notification,
    ) -> ZbusNotificationHandle {
        ZbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) -> Result<()> {
        wait_for_action_signal(&self.connection, self.id, invocation_closure)
    }

    pub fn close(self) -> Result<()> {
        self.connection.call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "CloseNotification",
            &(self.id),
        )?;
        Ok(())
    }

    pub fn on_close<F>(self, closure: F) -> Result<()>
    where
        F: FnOnce(CloseReason),
    {
        self.wait_for_action(|action: &ActionResponse| {
            if let ActionResponse::Closed(reason) = action {
                closure(*reason);
            }
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.id = send_notificaion_via_connection(&self.notification, self.id, &self.connection)?;
        Ok(())
    }
}

pub fn send_notificaion_via_connection(
    notification: &Notification,
    id: u32,
    connection: &Connection,
) -> Result<u32> {
    let reply: u32 = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "Notify",
            &(
                &notification.appname,
                id,
                &notification.icon,
                &notification.summary,
                &notification.body,
                &notification.actions,
                crate::hints::hints_to_map(notification),
                notification.timeout.into_i32(),
            ),
        )?
        .body()
        .deserialize()?;
    Ok(reply)
}

pub fn connect_and_send_notification(
    notification: &Notification,
) -> Result<ZbusNotificationHandle> {
    let connection = zbus::blocking::Connection::session()?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notificaion_via_connection(notification, inner_id, &connection)?;
    Ok(ZbusNotificationHandle::new(
        id,
        connection,
        notification.clone(),
    ))
}

pub fn get_capabilities() -> Result<Vec<String>> {
    let connection = zbus::blocking::Connection::session()?;
    let info: Vec<String> = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetCapabilities",
            &(),
        )?
        .body()
        .deserialize()?;

    Ok(info)
}

pub fn get_server_information() -> Result<xdg::ServerInformation> {
    let connection = zbus::blocking::Connection::session()?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetServerInformation",
            &(),
        )?
        .body()
        .deserialize()?;

    Ok(info)
}

fn wait_for_action_signal(
    connection: &Connection,
    id: u32,
    handler: impl ActionResponseHandler,
) -> Result<()> {
    let proxy = zbus::blocking::fdo::DBusProxy::new(connection)?;
    proxy.add_match_rule(
        MatchRule::builder()
            .interface("org.freedesktop.Notifications")?
            .member("ActionInvoked")?
            .build(),
    )?;
    proxy.add_match_rule(
        MatchRule::builder()
            .interface("org.freedesktop.Notifications")?
            .member("NotificationClosed")?
            .build(),
    )?;

    for msg in zbus::blocking::MessageIterator::from(connection).flatten() {
        if zbus::MessageType::Signal == msg.header().message_type() {
            if let Some(name) = msg.header().member() {
                if name == "ActionInvoked" {
                    match msg.body().deserialize::<(u32, String)>() {
                        Ok((nid, action)) if nid == id => {
                            handler.call(&ActionResponse::Custom(&action));
                            break;
                        }
                        _ => {}
                    }
                } else if name == "NotificationClosed" {
                    match msg.body().deserialize::<(u32, u32)>() {
                        Ok((nid, reason)) if nid == id => {
                            handler.call(&ActionResponse::Closed(reason.into()));
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
