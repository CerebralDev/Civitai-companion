fn main() {
    let mut commands = vec![
        "get_snapshot",
        "configure_api_key",
        "remove_api_key",
        "test_connection",
        "sync_now",
        "update_settings",
        "mark_notification_read",
        "send_thanks",
        "mark_all_notifications_read",
        "get_buzz_transactions",
        "preview_sound",
        "fetch_civitai_image",
        "open_civitai_url",
        "export_preferences",
        "import_preferences",
        "reset_cached_data",
        "clear_account_data",
    ];
    if std::env::var_os("CARGO_FEATURE_TEST_PANEL").is_some() {
        commands.extend(["open_test_panel", "send_test_notification"]);
    }
    let manifest = tauri_build::AppManifest::new().commands(commands.leak());
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(manifest))
        .expect("tauri-build failed")
}
