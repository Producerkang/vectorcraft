//! Update checks against the personal fork's releases.
//!
//! The upstream Windows self-updater requires an Authenticode-signed executable
//! from the same publisher. Our unsigned builds deliberately offer only a
//! download link; no signature-verification bypass or in-place replacement.

use serde_json::Value;

use crate::VectorcraftApp;

#[cfg(not(target_arch = "wasm32"))]
use craft_update::updater::Status;
#[cfg(not(target_arch = "wasm32"))]
use craft_update::{CheckResultDialog, CheckResultResponse, Release, StartupPrompt, StartupPromptResponse};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{Receiver, TryRecvError, channel};

pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    started: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    #[cfg(not(target_arch = "wasm32"))]
    pending: Option<Receiver<Result<Option<Release>, String>>>,
    #[cfg(not(target_arch = "wasm32"))]
    status: Status,
    #[cfg(not(target_arch = "wasm32"))]
    prompt: Option<StartupPrompt>,
    /// Whether Help ▸ Check for Updates is displaying a result.
    pub(crate) manual_window_open: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            started: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            pending: None,
            #[cfg(not(target_arch = "wasm32"))]
            status: Status::Idle,
            #[cfg(not(target_arch = "wasm32"))]
            prompt: None,
            manual_window_open: false,
            #[cfg(target_arch = "wasm32")]
            status_message: None,
        }
    }
}

impl VectorcraftApp {
    /// Enable one startup check against the custom release channel.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

pub(crate) fn check(app: &mut VectorcraftApp) -> Result<Value, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.check_requested = true;
        app.updates_rt.status = Status::Checking;
        Ok(Value::Null)
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some("Update check is not supported in the web browser.".into());
        Ok(Value::Null)
    }
}

/// Always create a download-only prompt. An unsigned build may not install itself.
#[cfg(not(target_arch = "wasm32"))]
fn download_prompt(release: Release) -> StartupPrompt {
    let mut prompt = StartupPrompt::new(&crate::UPDATE_APP, release, env!("CARGO_PKG_VERSION"));
    prompt.is_self_updating = false;
    prompt.status_message = Some(
        "Personal build: open the release page, download the Windows portable ZIP and replace the old folder after closing the app. Automatic installation requires trusted code signing.".into(),
    );
    prompt
}

pub(crate) fn tick(app: &mut VectorcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        let requested = std::mem::take(&mut rt.check_requested);
        let startup = rt.startup_check && !rt.started;
        if requested || startup {
            rt.started = true;
            if rt.pending.is_none() {
                rt.status = Status::Checking;
                let version = env!("CARGO_PKG_VERSION").to_owned();
                let repaint = ctx.clone();
                let (tx, rx) = channel();
                match std::thread::Builder::new().name("custom-update-check".into()).spawn(move || {
                    let answer = craft_update::Http::new("VectorCraft/CustomUpdate")
                        .and_then(|http| craft_update::check_for_update(&crate::UPDATE_APP, &http, &version));
                    let _ = tx.send(answer);
                    repaint.request_repaint();
                }) {
                    Ok(_) => rt.pending = Some(rx),
                    Err(e) => rt.status = Status::Failed(format!("could not start update check ({e})")),
                }
            }
        }
        if let Some(rx) = &rt.pending {
            let result = match rx.try_recv() {
                Ok(result) => Some(result),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => Some(Err("update check stopped unexpectedly".into())),
            };
            if let Some(result) = result {
                rt.pending = None;
                match result {
                    Ok(Some(release)) => {
                        rt.prompt = Some(download_prompt(release));
                        rt.status = Status::Idle;
                        rt.manual_window_open = false;
                    }
                    Ok(None) => rt.status = Status::UpToDate,
                    Err(e) => {
                        log::warn!("personal update check failed: {e}");
                        rt.status = Status::Failed(e);
                    }
                }
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

pub(crate) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut release_url = None;
        let rt = &mut app.updates_rt;
        if let Some(prompt) = rt.prompt.as_mut() {
            match prompt.show(ctx) {
                StartupPromptResponse::Pending => {}
                StartupPromptResponse::Later => rt.prompt = None,
                StartupPromptResponse::Download => {
                    release_url = Some(prompt.release.html_url.clone());
                    rt.prompt = None;
                }
                // Defensive fail-closed behavior: never self-install an unsigned build.
                StartupPromptResponse::UpdateAndRestart => {
                    rt.status = Status::Failed("Automatic installation is disabled for this unsigned personal build.".into());
                    rt.prompt = None;
                    rt.manual_window_open = true;
                }
            }
        }
        if rt.manual_window_open {
            match CheckResultDialog::new(&crate::UPDATE_APP, env!("CARGO_PKG_VERSION")).show(ctx, &rt.status) {
                CheckResultResponse::None => {}
                CheckResultResponse::Close => rt.manual_window_open = false,
                CheckResultResponse::CheckAgain => rt.check_requested = true,
            }
        }
        if let Some(url) = release_url {
            app.open_url(&url);
        }
    }

    #[cfg(target_arch = "wasm32")]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        egui::Window::new(tl!("VectorCraft Updates"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                if ui.button(tl!("OK")).clicked() {
                    app.updates_rt.manual_window_open = false;
                }
            });
        if !open {
            app.updates_rt.manual_window_open = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        assert!(!rt.manual_window_open);
        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(!rt.startup_check);
            assert!(!rt.started);
            assert!(rt.pending.is_none());
            assert!(rt.prompt.is_none());
            assert_eq!(rt.status, Status::Idle);
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn private_channel_never_offers_unsigned_self_install() {
        assert_eq!(crate::UPDATE_APP.repo, "Producerkang/vectorcraft");
        let release = Release {
            tag: "v100.0.2".into(),
            version: craft_update::parse_version("100.0.2"),
            name: "Custom VectorCraft".into(),
            html_url: "https://github.com/Producerkang/vectorcraft/releases/tag/v100.0.2".into(),
            ..Default::default()
        };
        let prompt = download_prompt(release);
        assert!(!prompt.is_self_updating);
        assert_eq!(prompt.app.repo, "Producerkang/vectorcraft");
    }
}
