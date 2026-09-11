const CRASH_REPORTER_PROCESS_ENV_VAR: &str = "_RELAY_CRASH_REPORTER_PROCESS";

/// Returns `true` if the current process is the crash reporting process.
pub fn is_crash_reporter_process() -> bool {
    cfg!(feature = "crash-handler") && std::env::var_os(CRASH_REPORTER_PROCESS_ENV_VAR).is_some()
}

/// Initializes a secondary watcher process for reporting fatal crashes.
///
/// This creates a secondary process, spawned from the same binary, communicating
/// with the original process via IPC. If the primary process crashes,
/// the secondary process will capture a minidump and report it to Sentry, delaying
/// the shutdown of the crashed process until the minidump has been uploaded.
///
/// In the crash reporter process this function never returns. The crash reporter
/// exits with status 1 if it fails to start.
#[cfg(feature = "crash-handler")]
pub fn init(config: &crate::SentryConfig, client: crate::sentry::Client) {
    let Some(db) = config._crash_db.clone() else {
        return;
    };

    let builder = sentry_rust_minidump::Builder::new()
        .server_env_var(CRASH_REPORTER_PROCESS_ENV_VAR)
        .crashes_dir(db.clone())
        .inherit_args(true)
        .process_name("relay-crash")
        .before_capture(|_scope, path| {
            crate::info!("Minidump captured ({})", path.display());
        })
        .flush_timeout(std::time::Duration::from_secs(15));

    // Make sure the implementations agree on the detection.
    debug_assert_eq!(
        is_crash_reporter_process(),
        builder.is_crash_reporter_process()
    );

    if !builder.is_crash_reporter_process() {
        crate::info!("Initializing crash handler in {}", db.display());
    }

    match builder.install(&client) {
        Ok(handle) => {
            crate::info!("Crash handler initialized");
            // Dropping the handle would detach the crash handler not catching any crashes.
            handle.leak();
        }
        Err(err) => {
            crate::warn!(
                error = &err as &dyn std::error::Error,
                "Failed to initialize crash reporting process"
            );
        }
    }
}
