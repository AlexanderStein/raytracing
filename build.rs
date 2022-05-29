use anyhow::Result;
use vergen::{vergen, Config, SemverKind};

fn main() -> Result<()> {
    let mut config = Config::default();
    *config.git_mut().branch_mut() = false;
    *config.git_mut().commit_count_mut() = false;
    *config.git_mut().commit_timestamp_mut() = false;
    *config.git_mut().rerun_on_head_change_mut() = true;
    *config.git_mut().semver_mut() = true;
    *config.git_mut().sha_mut() = false;
    // Include lightweight tags as well
    *config.git_mut().semver_kind_mut() = SemverKind::Lightweight;
    // Add a `-dirty` flag to the SEMVER output
    *config.git_mut().semver_dirty_mut() = Some("-dirty");

    vergen(config)
}
