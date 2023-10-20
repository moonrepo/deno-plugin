use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

#[test]
fn registers_metadata() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("deno-test", sandbox.path());

    assert_eq!(
        plugin.register_tool(ToolMetadataInput::default()),
        ToolMetadataOutput {
            name: "Deno".into(),
            plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
            self_upgrade_commands: vec!["upgrade".into()],
            ..ToolMetadataOutput::default()
        }
    );
}
