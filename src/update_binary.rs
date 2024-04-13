use anyhow::Result;
use self_update::cargo_crate_version;

pub fn update_binary() -> Result<()> {
    let _ = self_update::backends::github::Update::configure()
        .repo_owner("mijinko17")
        .repo_name("neovim-container")
        .bin_name("neovim-container")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(())
}
