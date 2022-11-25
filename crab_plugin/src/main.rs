//! plugin macros usage example.
extern crate clightningrpc_plugin_macros;
use clightningrpc_plugin::add_rpc;
use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin::types::LogLevel;
use clightningrpc_plugin_macros::{add_plugin_rpc, notification, rpc_method};
use serde_json::{json, Value};
use std::str::FromStr;

mod plugin;
use crate::plugin::{CrabCommand, CrabState};

#[rpc_method(
    rpc_name = "crab",
    description = "Crab is a dispach method to run crab subcommand"
)]
pub fn foo_rpc(_plugin: &Plugin<CrabState>, _request: &Value) -> Value {
    let command = _request.get("cmd").unwrap().as_str().unwrap();
    match CrabCommand::from_str(command).unwrap() {
        CrabCommand::Subscribe => {}
        CrabCommand::Unsubscribe => {}
    }
    json!({"is_dynamic": _plugin.dynamic, "rpc_request": _request})
}

#[notification(on = "rpc_command")]
fn on_rpc(_plugin: &Plugin<CrabState>, _request: &Value) {
    _plugin.log(LogLevel::Info, "received an RPC notification");
}

fn main() {
    // as fist step you need to make a new plugin instance
    // more docs about Plugin struct is provided under the clightning_plugin crate
    let mut plugin = Plugin::new(CrabState {}, true);

    // The macros helper that help to register a RPC method with the name
    // without worry about all the rules of the library
    add_plugin_rpc!(plugin, "crab");

    plugin.start();
}
