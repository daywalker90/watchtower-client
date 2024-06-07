use cln_plugin::options::DefaultIntegerConfigOption;

// Collection of ENV variable names and values
pub const TOWERS_DATA_DIR: &str = "TOWERS_DATA_DIR";
pub const DEFAULT_TOWERS_DATA_DIR: &str = ".watchtower";

/// Collections of plugin option names, default values and descriptions

pub const WT_PORT: DefaultIntegerConfigOption =
    DefaultIntegerConfigOption::new_i64_with_default("watchtower-port", 9814, "tower API port");
pub const WT_MAX_RETRY_TIME: DefaultIntegerConfigOption =
    DefaultIntegerConfigOption::new_i64_with_default(
        "watchtower-max-retry-time",
        3600,
        "for how long (in seconds) a retry strategy will try to reach a \
     temporary unreachable tower before giving up. Defaults to 1 hour",
    );
pub const WT_AUTO_RETRY_DELAY: DefaultIntegerConfigOption =
    DefaultIntegerConfigOption::new_i64_with_default(
        "watchtower-auto-retry-delay",
        28800,
        "how long (in seconds) a retrier will wait before auto-retrying a failed tower. \
     Defaults to once every 8 hours",
    );
pub const DEV_WT_MAX_RETRY_INTERVAL: DefaultIntegerConfigOption =
    DefaultIntegerConfigOption::new_i64_with_default(
        "dev-watchtower-max-retry-interval",
        900,
        "maximum length (in seconds) for a retry interval. Defaults to 15 min",
    );

/// Collections of rpc method names and descriptions

pub const RPC_REGISTER_TOWER: &str = "registertower";
pub const RPC_REGISTER_TOWER_DESC: &str =
    "Registers the client public key (user id) with the tower";
pub const RPC_GET_REGISTRATION_RECEIPT: &str = "getregistrationreceipt";
pub const RPC_GET_REGISTRATION_RECEIPT_DESC: &str =
    "Gets the latest registration receipt given a tower id";
pub const RPC_GET_APPOINTMENT: &str = "getappointment";
pub const RPC_GET_APPOINTMENT_DESC: &str =
    "Gets appointment data from the tower given a tower id and a locator";
pub const RPC_GET_APPOINTMENT_RECEIPT: &str = "getappointmentreceipt";
pub const RPC_GET_APPOINTMENT_RECEIPT_DESC: &str =
    "Gets a (local) appointment receipt given a tower id and a locator";
pub const RPC_GET_SUBSCRIPTION_INFO: &str = "getsubscriptioninfo";
pub const RPC_GET_SUBSCRIPTION_INFO_DESC: &str =
    "Gets the subscription information directly from the tower";
pub const RPC_LIST_TOWERS: &str = "listtowers";
pub const RPC_LIST_TOWERS_DESC: &str = "Lists all registered towers";
pub const RPC_GET_TOWER_INFO: &str = "gettowerinfo";
pub const RPC_GET_TOWER_INFO_DESC: &str = "Shows the info about a tower given a tower id";
pub const RPC_RETRY_TOWER: &str = "retrytower";
pub const RPC_RETRY_TOWER_DESC: &str =
    "Retries to send pending appointment to an unreachable tower";
pub const RPC_ABANDON_TOWER: &str = "abandontower";
pub const RPC_ABANDON_TOWER_DESC: &str = "Forgets about a tower and wipes all local data";
pub const RPC_PING: &str = "pingtower";
pub const RPC_PING_DESC: &str = "Polls the tower to check if it is online";

/// Collections of hook names

pub const HOOK_COMMITMENT_REVOCATION: &str = "commitment_revocation";
