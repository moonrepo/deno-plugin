use crate::config::DenoPluginConfig;
use extism_pdk::*;
use proto_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Deno";
static BIN: &str = "deno";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        self_upgrade_commands: vec!["upgrade".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec![".dvmrc".into()],
        ignore: vec![],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/denoland/deno")?;

    let tags = tags
        .iter()
        .filter_map(|tag| tag.strip_prefix('v').map(|tag| tag.to_owned()))
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

// https://docs.deno.com/runtime/manual/references/contributing/building_from_source
#[plugin_fn]
pub fn build_instructions(
    Json(_): Json<BuildInstructionsInput>,
) -> FnResult<Json<BuildInstructionsOutput>> {
    let env = get_host_environment()?;

    let mut output = BuildInstructionsOutput {
        source: SourceLocation::Git(GitSource {
            url: "https://github.com/denoland/deno.git".into(),
            reference: "main".into(), // TODO
            submodules: true,
        }),
        help_url: Some(
            "https://docs.deno.com/runtime/manual/references/contributing/building_from_source"
                .into(),
        ),
        requirements: vec![
            BuildRequirement::CommandExistsOnPath("cargo".into()),
            BuildRequirement::GitVersion(VersionReq::parse(">=2.19.2")?),
            BuildRequirement::PythonVersion(VersionReq::parse(">=3")?),
            BuildRequirement::XcodeCommandLineTools,
        ],
        system_dependencies: vec![
            // Linux
            SystemDependency::for_os("cmake", HostOS::Linux),
            SystemDependency::for_os("libglib2.0-dev", HostOS::Linux),
            SystemDependency::for_os("protobuf-compiler", HostOS::Linux),
            // macOS
            SystemDependency::for_os("cmake", HostOS::MacOS),
            SystemDependency::for_os_arch("llvm", HostOS::MacOS, HostArch::Arm64),
            SystemDependency::for_os("protobuf", HostOS::MacOS),
            // Windows
        ],
        ..Default::default()
    };

    match env.os {
        HostOS::MacOS => {}
        HostOS::Windows => {
            output.requirements.extend(vec![
                BuildRequirement::GitConfigSetting(
                    "core.symlinks".into(),
                    "true".into(),
                ),
                BuildRequirement::ManualIntercept(
                    "https://docs.deno.com/runtime/manual/references/contributing/building_from_source#native-compilers-and-linkers".into(),
                ),
            ]);

            // TODO download protobuf
        }
        // Not sure if these apply to all Linux based...
        _ => {
            output.instructions.extend(vec![
                BuildInstruction::RequestScript("https://apt.llvm.org/llvm.sh".into()),
                BuildInstruction::MakeExecutable("llvm.sh".into()),
                BuildInstruction::RunCommand(CommandInstruction::new("./llvm.sh", ["16"])),
                BuildInstruction::RemoveFile("./llvm.sh".into()),
            ]);
        }
    };

    // These must come last as it's the actual command to build the binary!
    let target_bin = env.os.get_exe_name("target/debug/deno");

    output.instructions.extend(vec![
        BuildInstruction::RunCommand(CommandInstruction::new("cargo", ["build", "-vv"])),
        BuildInstruction::MoveFile(target_bin.into(), ".".into()),
        BuildInstruction::RemoveDir("target".into()),
    ]);

    Ok(Json(output))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = &input.context.version;

    let arch = match env.arch {
        HostArch::Arm64 => "aarch64",
        HostArch::X64 => "x86_64",
        _ => unreachable!(),
    };

    let filename = match env.os {
        HostOS::Linux => format!("deno-{arch}-unknown-linux-gnu.zip"),
        HostOS::MacOS => format!("deno-{arch}-apple-darwin.zip"),
        HostOS::Windows => format!("deno-{arch}-pc-windows-msvc.zip"),
        _ => unreachable!(),
    };

    let download_url = if version.is_canary() {
        let hash = fetch_url_text("https://dl.deno.land/canary-latest.txt")?;

        format!("https://dl.deno.land/canary/{}/{filename}", hash.trim())
    } else if version.is_latest() {
        let tag = fetch_url_text("https://dl.deno.land/release-latest.txt")?;

        format!("https://dl.deno.land/release/{}/{filename}", tag.trim())
    } else {
        let config = get_tool_config::<DenoPluginConfig>()?;

        config
            .dist_url
            .replace("{version}", &version.to_string())
            .replace("{file}", &filename)
    };

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs: vec![
            "$DENO_INSTALL_ROOT/bin".into(),
            "$DENO_HOME/bin".into(),
            "$HOME/.deno/bin".into(),
        ],
        primary: Some(ExecutableConfig::new(env.os.get_exe_name(BIN))),
        ..LocateExecutablesOutput::default()
    }))
}
