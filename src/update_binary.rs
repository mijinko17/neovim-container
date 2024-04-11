use self_update::cargo_crate_version;

pub fn update_binary() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("mijinko17")
        .repo_name("neovim-container")
        .bin_name("neovim-container")
        .target(BINARY_TARGET)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}

#[cfg(target_os = "linux")]
static BINARY_TARGET: &str = "x86_64-unknown-linux-musl";
#[cfg(target_os = "macos")]
static BINARY_TARGET: &str = "-x86_64-apple-darwin";
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
static BINARY_TARGET: &str = "x86_64-unknown-linux-musl";
