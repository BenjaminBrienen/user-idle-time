use core::time::Duration;

use dbus::blocking::Connection;

use crate::error::Error;

const SCREENSAVERS: &[&[&str]] = &[
    &[
        "org.freedesktop.ScreenSaver",
        "/org/freedesktop/ScreenSaver",
        "org.freedesktop.ScreenSaver",
    ],
    &[
        "org.gnome.ScreenSaver",
        "/org/gnome/ScreenSaver",
        "org.gnome.ScreenSaver",
    ],
    &[
        "org.kde.ScreenSaver",
        "/org/kde/ScreenSaver",
        "org.kde.ScreenSaver",
    ],
];

/// Get the idle time of a user.
///
/// # Panics
///
/// Panics if a system call fails or if time flows backwards.
pub fn get_idle_time() -> Result<Duration, Error> {
    for screensaver in SCREENSAVERS {
        let Ok(conn) = Connection::new_session() else {
            continue;
        };

        let proxy = conn.with_proxy(screensaver[0], screensaver[1], Duration::from_millis(5000));

        let (time,): (u32,) = match proxy.method_call(screensaver[2], "GetActiveTime", ()) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // freedesktop seems to return the time in milliseconds??
        if screensaver[0] == "org.freedesktop.ScreenSaver" {
            return Ok(Duration::from_millis(u64::from(time)));
        }

        return Ok(Duration::from_secs(u64::from(time)));
    }

    Err(Error::new("No screensaver available"))
}
