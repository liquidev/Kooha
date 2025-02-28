use ashpd::zbus;
use gtk::glib;

use std::{cmp, path::Path};

const MAX_THREAD_COUNT: u32 = 64;

pub fn round_to_even(number: f64) -> i32 {
    number as i32 / 2 * 2
}

pub fn ideal_thread_count() -> u32 {
    let num_processors = glib::num_processors();
    cmp::min(num_processors, MAX_THREAD_COUNT)
}

pub fn check_if_accessible(path: &Path) -> bool {
    let home_folder = glib::home_dir();
    let is_in_home_folder = path.starts_with(&home_folder);

    is_in_home_folder && path != home_folder
}

pub fn set_raise_active_window_request(is_raised: bool) -> anyhow::Result<()> {
    shell_window_eval("make_above", is_raised)?;
    shell_window_eval("stick", is_raised)?;
    Ok(())
}

fn shell_window_eval(method: &str, is_enabled: bool) -> anyhow::Result<()> {
    let reverse_keyword = if is_enabled { "" } else { "un" };
    let command = format!(
        "global.display.focus_window.{}{}()",
        reverse_keyword, method
    );

    let connection = zbus::Connection::session()?;
    let reply = connection.call_method(
        Some("org.gnome.Shell"),
        "/org/gnome/Shell",
        Some("org.gnome.Shell"),
        "Eval",
        &command,
    )?;
    let (is_success, message) = reply.body::<(bool, String)>()?;

    if !is_success {
        anyhow::bail!(message);
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn odd_round_to_even() {
        assert_eq!(round_to_even(3.0), 2);
        assert_eq!(round_to_even(99.0), 98);
    }

    #[test]
    fn even_round_to_even() {
        assert_eq!(round_to_even(50.0), 50);
        assert_eq!(round_to_even(4.0), 4);
    }

    #[test]
    fn float_round_to_even() {
        assert_eq!(round_to_even(5.3), 4);
        assert_eq!(round_to_even(2.9), 2);
    }

    #[test]
    fn check_if_accessible_in_home() {
        let downloads_folder = glib::user_special_dir(glib::UserDirectory::Downloads);
        assert!(check_if_accessible(&downloads_folder));
    }

    #[test]
    fn check_if_accessible_not_in_home() {
        let random_dir = Path::new("/dev/sda");
        assert!(!check_if_accessible(random_dir));
    }

    #[test]
    fn check_if_accessible_home() {
        let home_dir = glib::home_dir();
        assert!(!check_if_accessible(&home_dir));
    }
}
