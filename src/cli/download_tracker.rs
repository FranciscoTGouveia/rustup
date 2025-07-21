use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use std::time::Duration;

use crate::dist::Notification as In;
use crate::notifications::Notification;
use crate::process::Process;
use crate::utils::Notification as Un;

/// Tracks download progress and displays information about it to a terminal.
///
/// *not* safe for tracking concurrent downloads yet - it is basically undefined
/// what will happen.
pub(crate) struct DownloadTracker {
    /// Whether we display progress or not.
    display_progress: bool,
    stdout_is_a_tty: bool,
    // Progress bar for the download.
    progress_bar: ProgressBar,
}

impl DownloadTracker {
    /// Creates a new DownloadTracker.
    pub(crate) fn new_with_display_progress(display_progress: bool, process: &Process) -> Self {
        Self {
            display_progress,
            stdout_is_a_tty: process.stdout().is_a_tty(process),
            progress_bar: ProgressBar::hidden(),
        }
    }

    pub(crate) fn handle_notification(&mut self, n: &Notification<'_>) -> bool {
        match *n {
            Notification::Install(In::Utils(Un::DownloadContentLengthReceived(content_len))) => {
                self.content_length_received(content_len);
                true
            }
            Notification::Install(In::Utils(Un::DownloadDataReceived(data))) => {
                if self.stdout_is_a_tty {
                    self.data_received(data.len());
                }
                true
            }
            Notification::Install(In::Utils(Un::DownloadFinished)) => {
                self.download_finished();
                true
            }
            Notification::Install(In::Utils(Un::DownloadPushUnit(_))) => true,
            Notification::Install(In::Utils(Un::DownloadPopUnit)) => true,

            _ => false,
        }
    }

    /// Sets the length for a new ProgressBar and gives it a style.
    pub(crate) fn content_length_received(&mut self, content_len: u64) {
        self.progress_bar.set_length(content_len);
        self.progress_bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{bar:40.blue}] {bytes}/{total_bytes} (ETA: {eta})",
            )
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "),
        );
    }

    /// Notifies self that data of size `len` has been received.
    pub(crate) fn data_received(&mut self, len: usize) {
        if self.progress_bar.is_hidden()
            && self.display_progress
            && self.stdout_is_a_tty
            && self.progress_bar.elapsed() >= Duration::from_secs(1)
        {
            self.progress_bar
                .set_draw_target(ProgressDrawTarget::stdout());
        }
        self.progress_bar.inc(len as u64);
    }

    /// Notifies self that the download has finished.
    pub(crate) fn download_finished(&mut self) {
        self.progress_bar.finish_and_clear();
        self.progress_bar = ProgressBar::hidden();
    }
}
