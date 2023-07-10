use extism_pdk::*;
use proto_pdk::*;
use serde::Deserialize;

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

    let arch = match input.env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        other => {
            return Err(PluginError::UnsupportedArchitecture {
                tool: NAME.into(),
                arch: format!("{:?}", other),
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
                platform: format!("{:?}", other),
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
        globals_lookup_dirs: vec![
            "$DENO_INSTALL_ROOT/bin".into(),
            "$DENO_HOME/bin".into(),
            "$HOME/.deno/bin".into(),
        ],
    }))
}

#[derive(Deserialize)]
pub struct TagEntry {
    name: String,
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut output = LoadVersionsOutput::default();
    let mut latest = Version::new(0, 0, 0);

    let response: Vec<TagEntry> = fetch_url("https://api.github.com/repos/denoland/deno/tags")?;
    let tags = response
        .iter()
        .filter_map(|entry| entry.name.strip_prefix('v'))
        .collect::<Vec<_>>();

    for tag in tags {
        let version = Version::parse(tag)?;

        if version > latest {
            latest = version.clone();
        }

        output.versions.push(version);
    }

    output.aliases.insert("latest".into(), latest);

    Ok(Json(output))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![".dvmrc".into()],
    }))
}
