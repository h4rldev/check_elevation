#![no_std]

use windows::Win32::{
    Foundation::HANDLE,
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ACCESS_MASK, TOKEN_ELEVATION, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};
/*
Checks if the current Windows process is elevated.
Returns true if the process is elevated, false if not.
Example:
```rust
use check_elevation::is_elevated;

fn main() {
    if is_elevated().expect("Failed to get elevation status.") {
        println!("Running as administrator.");
    } else {
        println!("Not running as administrator.");
    }
}
```
*/

pub fn is_elevated() -> windows::core::Result<bool> {
    unsafe {
        let mut h_token: HANDLE = HANDLE(0 as _);
        let result = OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ACCESS_MASK(TOKEN_QUERY.0),
            &mut h_token,
        );
        match result {
            Ok(_) => {
                let mut token_elevation: TOKEN_ELEVATION = core::mem::zeroed();
                let mut return_length = 0;

                match GetTokenInformation(
                    h_token,
                    TokenElevation,
                    Some(&mut token_elevation as *mut _ as *mut _),
                    core::mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) {
                    Ok(_) => {
                        if token_elevation.TokenIsElevated != 0 {
                            Ok(true)
                        } else {
                            Ok(false)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }
}
