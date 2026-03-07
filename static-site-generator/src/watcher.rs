use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub struct Watcher {
    watcher: notify::RecommendedWatcher,
    rx: mpsc::Receiver<Result<Event, notify::Error>>,
}

impl Watcher {
    pub fn new() -> Result<Self> {
        let (tx, rx) = mpsc::channel();

        let watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;

        Ok(Self { watcher, rx })
    }

    pub fn watch(&mut self, path: &Path) -> Result<()> {
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn check_for_changes(&self, timeout: Duration) -> Option<Vec<Event>> {
        let mut events = Vec::new();
        let deadline = std::time::Instant::now() + timeout;

        while let Ok(result) = self.rx.recv_timeout(
            deadline.saturating_duration_since(std::time::Instant::now())
        ) {
            match result {
                Ok(event) => events.push(event),
                Err(e) => eprintln!("Watch error: {}", e),
            }

            if std::time::Instant::now() >= deadline {
                break;
            }
        }

        if events.is_empty() {
            None
        } else {
            Some(events)
        }
    }
}
