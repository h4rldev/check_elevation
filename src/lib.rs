//! Checks if the current Windows process is elevated.
//! Returns true if the process is elevated, false if not.
//! ## Example
//! ```rust
//! use check_elevation::is_elevated;
//!
//! if is_elevated().expect("Failed to get elevation status.") {
//!     println!("Running as administrator.");
//! } else {
//!     eprintln!("Not running as administrator.");
//! }
//! ```

#![no_std]

use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, WIN32_ERROR},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

pub fn is_elevated() -> Result<bool, WIN32_ERROR> {
    unsafe {
        let mut h_token: HANDLE = 0;
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token) == 0 {
            let error = GetLastError();
            CloseHandle(h_token);
            return Err(error);
        }

        let mut token_elevation: TOKEN_ELEVATION = core::mem::zeroed();
        if GetTokenInformation(
            h_token,
            TokenElevation,
            core::ptr::addr_of_mut!(token_elevation).cast(),
            core::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut 0,
        ) == 0
        {
            let error = GetLastError();
            CloseHandle(h_token);
            return Err(error);
        }

        CloseHandle(h_token);
        Ok(token_elevation.TokenIsElevated != 0)
    }
}
