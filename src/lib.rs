use windows::Win32::{
    Foundation::HANDLE,
    Security::{
        GetTokenInformation, TokenElevation, TOKEN_ACCESS_MASK, TOKEN_ELEVATION, TOKEN_QUERY,
    },
    System::Threading::{GetCurrentProcess, OpenProcessToken},
};

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
                let mut token_elevation: TOKEN_ELEVATION = std::mem::zeroed();
                let mut return_length = 0;

                match GetTokenInformation(
                    h_token,
                    TokenElevation,
                    Some(&mut token_elevation as *mut _ as *mut _),
                    std::mem::size_of::<TOKEN_ELEVATION>() as u32,
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
