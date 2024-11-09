use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[derive(Clone)]
pub struct ScanProgress {
    scan_pb: ProgressBar,
    results_pb: ProgressBar,
}

impl ScanProgress {
    pub fn new(total_batches: u64) -> Self {
        let mp = MultiProgress::new();

        let scan_pb = mp.add(ProgressBar::new(total_batches));
        scan_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} batches ({per_sec})")
                .unwrap(),
        );

        let results_pb = mp.add(ProgressBar::new(64));
        results_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.green/white} {pos} open ports found")
                .unwrap(),
        );

        ScanProgress {
            scan_pb,
            results_pb,
        }
    }

    pub fn inc_scanned(&self) {
        self.scan_pb.inc(1);
    }

    pub fn inc_found(&self) {
        self.results_pb.inc(1);
    }

    pub fn finish(&self) {
        self.scan_pb.finish();
        self.results_pb.finish();
    }
}
