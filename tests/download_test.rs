use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;
use std::collections::HashMap;

generate_download_install_tests!("deno-test", "1.30.0");

// Deno doesn't provide canary builds for MacOS M1
#[cfg(not(target_os = "macos"))]
mod canary {
    use super::*;

    generate_download_install_tests!("deno-test", "canary");
}

#[test]
#[should_panic(expected = "Unable to install Deno, unsupported architecture arm64 for linux.")]
fn doesnt_support_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Linux, HostArch::Arm64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-aarch64-unknown-linux-gnu.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-aarch64-unknown-linux-gnu.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Linux, HostArch::X64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-x86_64-unknown-linux-gnu.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-x86_64-unknown-linux-gnu.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::Arm64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-aarch64-apple-darwin.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-aarch64-apple-darwin.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::MacOS, HostArch::X64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-x86_64-apple-darwin.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-x86_64-apple-darwin.zip".into(),
            ..Default::default()
        }
    );
}

#[test]
#[should_panic(expected = "Unable to install Deno, unsupported architecture arm64 for windows.")]
fn doesnt_support_windows_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Windows, HostArch::Arm64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-aarch64-pc-windows-msvc.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-aarch64-pc-windows-msvc.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Windows, HostArch::X64)]),
    );

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            context: ToolContext {
                version: VersionSpec::parse("1.2.0").unwrap(),
                ..Default::default()
            },
            ..Default::default()
        }),
        DownloadPrebuiltOutput {
            download_name: Some("deno-x86_64-pc-windows-msvc.zip".into()),
            download_url: "https://dl.deno.land/release/v1.2.0/deno-x86_64-pc-windows-msvc.zip"
                .into(),
            ..Default::default()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Linux, HostArch::Arm64)]),
    );

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("deno".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin_with_config(
        "deno-test",
        sandbox.path(),
        HashMap::from_iter([map_config_environment(HostOS::Windows, HostArch::X64)]),
    );

    assert_eq!(
        plugin
            .locate_executables(LocateExecutablesInput {
                context: ToolContext {
                    version: VersionSpec::parse("1.2.0").unwrap(),
                    ..Default::default()
                },
            })
            .primary
            .unwrap()
            .exe_path,
        Some("deno.exe".into())
    );
}
