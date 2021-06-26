use crate::wintun_raw;
use log::*;
use widestring::U16CStr;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn set_logger(wintun: &Arc<wintun_raw::wintun>, f: wintun_raw::WINTUN_LOGGER_CALLBACK) {
    unsafe { wintun.WintunSetLogger(f) };
}

pub fn get_running_driver_version(wintun: &Arc<wintun_raw::wintun>) -> u32 {
    unsafe { wintun.WintunGetRunningDriverVersion() }
}

static SET_LOGGER: AtomicBool = AtomicBool::new(false);

//unsafe extern "C" fn(Level: WINTUN_LOGGER_LEVEL, Message: *const WCHAR)
pub extern "C" fn default_logger(
    level: wintun_raw::WINTUN_LOGGER_LEVEL,
    message: *const wintun_raw::WCHAR,
) {
    //Winton will always give us a valid UTF16 null termineted string
    let msg = unsafe { U16CStr::from_ptr_str(message) };
    let utf8_msg = msg.to_string_lossy();
    match level {
        wintun_raw::WINTUN_LOGGER_LEVEL_WINTUN_LOG_INFO => info!("WinTun: {}", utf8_msg),
        wintun_raw::WINTUN_LOGGER_LEVEL_WINTUN_LOG_WARN => warn!("WinTun: {}", utf8_msg),
        wintun_raw::WINTUN_LOGGER_LEVEL_WINTUN_LOG_ERR => error!("WinTun: {}", utf8_msg),
        _ => error!("WinTun: {} (with invalid log level {})", utf8_msg, level),
    }
}

pub(crate) fn set_default_logger_if_unset(wintun: &Arc<wintun_raw::wintun>) {
    if !SET_LOGGER.load(Ordering::SeqCst) {
        set_logger(wintun, Some(default_logger));
        SET_LOGGER.store(true, Ordering::SeqCst);
    }
}


