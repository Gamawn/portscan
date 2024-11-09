use portscan::ui::ScanProgress;

#[test]
fn test_progress_updates() {
    let progress = ScanProgress::new(10);

    progress.inc_scanned();
    progress.inc_found();
    progress.finish();
}
