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
    let version = input.env.version;

    // How granular do we want to get here?
    if input.env.arch == HostArch::Arm64 && input.env.os != HostOS::MacOS {
        return Err(PluginError::UnsupportedArchitecture {
            tool: NAME.into(),
            arch: input.env.arch.to_string(),
        })?;
    }

    let arch = match input.env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        other => {
            return Err(PluginError::UnsupportedArchitecture {
                tool: NAME.into(),
                arch: other.to_string(),
            })?;
        }
    };

    let prefix = match input.env.os {
        HostOS::Linux => format!("deno-{arch}-unknown-linux-gnu"),
        HostOS::MacOS => format!("deno-{arch}-apple-darwin"),
        HostOS::Windows => format!("deno-{arch}-pc-windows-msvc"),
        other => {
            return Err(PluginError::UnsupportedPlatform {
                tool: NAME.into(),
                platform: other.to_string(),
            })?;
        }
    };

    let filename = format!("{prefix}.zip");

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
        bin_path: Some(if input.env.os == HostOS::Windows {
            format!("{}.exe", BIN)
        } else {
            BIN.into()
        }),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec![
            "$DENO_INSTALL_ROOT".into(),
            "$DENO_HOME/bin".into(),
            "$HOME/.deno/bin".into(),
        ],
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
