use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "Deno";
static BIN: &str = "deno";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    check_supported_os_and_arch(
        NAME,
        &input.env,
        permutations! [
            HostOS::Linux => [HostArch::X64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = input.env.version;

    let arch = match input.env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        _ => unreachable!(),
    };

    let filename = match input.env.os {
        HostOS::Linux => format!("deno-{arch}-unknown-linux-gnu.zip"),
        HostOS::MacOS => format!("deno-{arch}-apple-darwin.zip"),
        HostOS::Windows => format!("deno-{arch}-pc-windows-msvc.zip"),
        _ => unreachable!(),
    };

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/denoland/deno/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    Ok(Json(LocateBinsOutput {
        bin_path: Some(format_bin_name(BIN, input.env.os).into()),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec![
            "$DENO_INSTALL_ROOT".into(),
            "$DENO_HOME/bin".into(),
            "$HOME/.deno/bin".into(),
        ],
        ..LocateBinsOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/denoland/deno")?;

    let tags = tags
        .iter()
        .filter(|t| !t.ends_with("^{}"))
        .filter_map(|t| t.strip_prefix('v').map(|t| t.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from_tags(&tags)?))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![".dvmrc".into()],
    }))
}
