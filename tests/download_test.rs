use proto_pdk::*;
use proto_pdk_test_utils::{create_plugin, generate_download_install_tests};
use starbase_sandbox::create_empty_sandbox;
use std::path::PathBuf;

generate_download_install_tests!("deno-test", "1.30.0");

#[test]
#[should_panic(expected = "Unable to install Deno, unsupported architecture arm64.")]
fn doesnt_support_linux_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::Linux,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-aarch64-unknown-linux-gnu.zip".into()),
            download_url:
                "https://github.com/denoland/deno/releases/download/v1.2.0/deno-aarch64-unknown-linux-gnu.zip"
                    .into()
        }
    );
}

#[test]
fn supports_linux_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::Linux,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-x86_64-unknown-linux-gnu.zip".into()),
            download_url: "https://github.com/denoland/deno/releases/download/v1.2.0/deno-x86_64-unknown-linux-gnu.zip".into()
        }
    );
}

#[test]
fn supports_macos_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::MacOS,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-aarch64-apple-darwin.zip".into()),
            download_url:
                "https://github.com/denoland/deno/releases/download/v1.2.0/deno-aarch64-apple-darwin.zip"
                    .into()
        }
    );
}

#[test]
fn supports_macos_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::MacOS,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-x86_64-apple-darwin.zip".into()),
            download_url: "https://github.com/denoland/deno/releases/download/v1.2.0/deno-x86_64-apple-darwin.zip".into()
        }
    );
}

#[test]
#[should_panic(expected = "Unable to install Deno, unsupported architecture arm64.")]
fn doesnt_support_windows_arm64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::Arm64,
                os: HostOS::Windows,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-aarch64-pc-windows-msvc.zip".into()),
            download_url: "https://github.com/denoland/deno/releases/download/v1.2.0/deno-aarch64-pc-windows-msvc.zip".into()
        }
    );
}

#[test]
fn supports_windows_x64() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin.download_prebuilt(DownloadPrebuiltInput {
            env: Environment {
                arch: HostArch::X64,
                os: HostOS::Windows,
                version: "1.2.0".into(),
                ..Default::default()
            }
        }),
        DownloadPrebuiltOutput {
            archive_prefix: None,
            bin_path: None,
            checksum_name: None,
            checksum_url: None,
            download_name: Some("deno-x86_64-pc-windows-msvc.zip".into()),
            download_url: "https://github.com/denoland/deno/releases/download/v1.2.0/deno-x86_64-pc-windows-msvc.zip".into()
        }
    );
}

#[test]
fn locates_unix_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::Arm64,
                    os: HostOS::Linux,
                    version: "1.2.0".into(),
                    ..Default::default()
                },
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("deno".into())
    );
}

#[test]
fn locates_windows_bin() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("bun-test", sandbox.path());

    assert_eq!(
        plugin
            .locate_bins(LocateBinsInput {
                env: Environment {
                    arch: HostArch::X64,
                    os: HostOS::Windows,
                    version: "1.2.0".into(),
                    ..Default::default()
                },
                tool_dir: PathBuf::new()
            })
            .bin_path,
        Some("deno.exe".into())
    );
}
