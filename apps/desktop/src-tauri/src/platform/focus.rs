use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::error::app_error::{AppError, AppResult};

const FOCUS_SETTLE_DELAY: Duration = Duration::from_millis(220);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontmostApp {
    pub name: String,
    pub pid: i32,
}

pub fn get_frontmost_app() -> AppResult<Option<FrontmostApp>> {
    #[cfg(target_os = "macos")]
    {
        return get_frontmost_app_macos();
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(None)
    }
}

pub fn activate_app(app: &FrontmostApp) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        return activate_app_macos(app);
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app;
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn get_frontmost_app_macos() -> AppResult<Option<FrontmostApp>> {
    let output = Command::new("osascript")
        .args([
            "-e",
            r#"tell application "System Events"
  set frontApp to first application process whose frontmost is true
  set appName to name of frontApp
  set appPid to unix id of frontApp
  return (appName as text) & linefeed & (appPid as text)
end tell"#,
        ])
        .output()
        .map_err(|error| AppError::Insertion(format!("failed to query frontmost app: {error}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Insertion(format!(
            "failed to query frontmost app: {}",
            stderr.trim()
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines().map(str::trim).filter(|line| !line.is_empty());
    let name = lines.next().unwrap_or("").to_string();
    let pid = lines
        .next()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0);

    if name.is_empty() || pid <= 0 {
        return Ok(None);
    }

    // Never treat TypeFlow as the insertion target.
    if name.to_lowercase().contains("typeflow") {
        return Ok(None);
    }

    Ok(Some(FrontmostApp { name, pid }))
}

#[cfg(target_os = "macos")]
fn activate_app_macos(app: &FrontmostApp) -> AppResult<()> {
    let script = format!(
        r#"tell application "System Events"
  set frontmost of first application process whose unix id is {pid} to true
end tell"#,
        pid = app.pid
    );

    let output = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|error| {
            AppError::Insertion(format!(
                "failed to activate app '{}' (pid {}): {error}",
                app.name, app.pid
            ))
        })?;

    if !output.status.success() {
        // Fallback: activate by application name.
        let fallback = format!(
            r#"tell application "{}" to activate"#,
            escape_applescript_string(&app.name)
        );
        let fallback_output = Command::new("osascript")
            .args(["-e", &fallback])
            .output()
            .map_err(|error| {
                AppError::Insertion(format!(
                    "failed to activate app '{}': {error}",
                    app.name
                ))
            })?;

        if !fallback_output.status.success() {
            let stderr = String::from_utf8_lossy(&fallback_output.stderr);
            return Err(AppError::Insertion(format!(
                "failed to activate app '{}' (pid {}): {}",
                app.name,
                app.pid,
                stderr.trim()
            )));
        }
    }

    thread::sleep(FOCUS_SETTLE_DELAY);
    Ok(())
}

#[cfg(target_os = "macos")]
fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
