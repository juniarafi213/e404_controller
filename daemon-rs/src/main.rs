use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{env, thread};

const MOD_DIR: &str = "/data/adb/modules/e404_controller";

struct DaemonConfig {
    conf_dir: String,
    sys_out: String,
    lock_file: String,
    gamelist_file: String,
    conf_file: String,
    ctrl_bin: String,
    apk_file: String,
}

impl DaemonConfig {
    fn new() -> Self {
        let base = if Path::new(MOD_DIR).exists() {
            MOD_DIR.to_string()
        } else {
            let exe_path = env::current_exe().unwrap_or_default();
            exe_path
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| MOD_DIR.to_string())
        };

        Self {
            conf_dir: format!("{}/config", base),
            sys_out: format!("{}/config/sysmon_out.txt", base),
            lock_file: format!("{}/config/sysmon.lock", base),
            gamelist_file: format!("{}/config/gamelist.txt", base),
            conf_file: format!("{}/config/e404.conf", base),
            ctrl_bin: format!("{}/bin/e404-ctrl", base),
            apk_file: format!("{}/bin/system_monitor.apk", base),
        }
    }

    fn read_conf_val(&self, key: &str, default: &str) -> String {
        if let Ok(file) = File::open(&self.conf_file) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == key {
                        return v.trim().to_string();
                    }
                }
            }
        }
        default.to_string()
    }

    fn load_games(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        if let Ok(file) = File::open(&self.gamelist_file) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    set.insert(trimmed.to_string());
                }
            }
        }
        set
    }

    fn run_ctrl(&self, args: &[&str]) {
        let _ = Command::new(&self.ctrl_bin).args(args).status();
    }
}

fn start_sysmon_process(cfg: &DaemonConfig) -> Option<Child> {
    if !Path::new(&cfg.apk_file).exists() {
        eprintln!("[e404-daemon] APK not found: {}", cfg.apk_file);
        return None;
    }

    let _ = fs::remove_file(&cfg.lock_file);

    Command::new("app_process")
        .arg(format!("-Djava.class.path={}", cfg.apk_file))
        .arg("/")
        .arg("--nice-name=E404SysMon")
        .arg("com.rem01gaming.systemmonitor.MainKt")
        .arg(&cfg.sys_out)
        .arg(&cfg.lock_file)
        .spawn()
        .ok()
}

fn process_sysmon_event(
    cfg: &DaemonConfig,
    in_game: &mut bool,
    in_battery_saver: &mut bool,
    last_pkg: &mut String,
    last_normal_prof: &mut String,
) {
    let content = match fs::read_to_string(&cfg.sys_out) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut current_pkg = String::new();
    let mut screen_awake = true;
    let mut bat_saver = false;

    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("focused_app ") {
            if let Some(p) = rest.split_whitespace().next() {
                current_pkg = p.to_string();
            }
        } else if let Some(rest) = line.strip_prefix("screen_awake ") {
            screen_awake = rest.trim() != "0";
        } else if let Some(rest) = line.strip_prefix("battery_saver ") {
            bat_saver = rest.trim() == "1";
        }
    }

    // 1. ROM Battery Saver has highest priority
    if bat_saver {
        if !*in_battery_saver {
            println!("[e404-daemon] ROM Battery Saver turned ON -> Applying BATTERY mode");
            cfg.run_ctrl(&["profile", "battery"]);
            *in_battery_saver = true;
            *in_game = false;
        }
        return;
    } else if *in_battery_saver {
        println!("[e404-daemon] ROM Battery Saver turned OFF -> Restoring {}", last_normal_prof);
        cfg.run_ctrl(&["profile", last_normal_prof]);
        *in_battery_saver = false;
    }

    // 2. Screen state
    if !screen_awake {
        if *in_game {
            println!("[e404-daemon] Screen off during game -> Setting balanced");
            cfg.run_ctrl(&["profile", "balanced"]);
            *in_game = false;
        }
        return;
    }

    if current_pkg.is_empty() || &current_pkg == last_pkg {
        return;
    }
    *last_pkg = current_pkg.clone();

    let is_auto_enabled = cfg.read_conf_val("AUTO_GAME", "1") == "1";
    if !is_auto_enabled {
        return;
    }

    let games = cfg.load_games();

    if games.contains(&current_pkg) {
        if !*in_game {
            let perf_lite = cfg.read_conf_val("PERF_LITE", "1") == "1";
            let target_profile = if perf_lite { "performance" } else { "benchmark" };

            println!("[e404-daemon] Game active: {} -> Setting {}", current_pkg, target_profile);
            cfg.run_ctrl(&["profile", target_profile]);
            let _ = fs::write(format!("{}/active_game", cfg.conf_dir), &current_pkg);
            *in_game = true;
        }
    } else if *in_game {
        println!("[e404-daemon] Exited game: {} foreground -> Restoring {}", current_pkg, last_normal_prof);
        cfg.run_ctrl(&["profile", last_normal_prof]);
        let _ = fs::remove_file(format!("{}/active_game", cfg.conf_dir));
        *in_game = false;
    }
}

fn main() {
    let cfg = DaemonConfig::new();
    let term = Arc::new(AtomicBool::new(false));

    signal_hook_init(term.clone());

    println!("[e404-daemon] Initializing event-driven Rust daemon (v1.2.0)...");
    let _ = fs::create_dir_all(&cfg.conf_dir);

    // Spawn Android system_monitor
    let mut sysmon_child = start_sysmon_process(&cfg);
    thread::sleep(Duration::from_millis(500));

    let games = cfg.load_games();
    println!("[e404-daemon] Loaded {} game package signatures", games.len());

    let mut in_game = false;
    let mut in_battery_saver = false;
    let mut last_pkg = String::new();
    let mut last_normal_prof = "balanced".to_string();

    // Setup inotify for zero CPU idle
    unsafe {
        let inotify_fd = libc::inotify_init1(libc::IN_CLOEXEC);
        if inotify_fd < 0 {
            eprintln!("[e404-daemon] Failed to initialize inotify, fallback to polling");
            run_poll_fallback(&cfg, term);
            return;
        }

        // Ensure sys_out exists
        if !Path::new(&cfg.sys_out).exists() {
            let _ = fs::write(&cfg.sys_out, "");
        }

        use std::ffi::CString;
        let c_path = CString::new(cfg.sys_out.clone()).unwrap();
        let wd = libc::inotify_add_watch(inotify_fd, c_path.as_ptr(), libc::IN_MODIFY | libc::IN_CLOSE_WRITE);
        if wd < 0 {
            eprintln!("[e404-daemon] Failed to watch sys_out, fallback to polling");
            libc::close(inotify_fd);
            run_poll_fallback(&cfg, term);
            return;
        }

        let mut buf = [0u8; 1024];

        while !term.load(Ordering::Relaxed) {
            // Blocking read from inotify_fd -> Kernel wakes us only on file write! 0.0% CPU!
            let len = libc::read(inotify_fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
            if len <= 0 {
                continue;
            }

            process_sysmon_event(&cfg, &mut in_game, &mut in_battery_saver, &mut last_pkg, &mut last_normal_prof);
        }

        libc::close(inotify_fd);
    }

    if let Some(mut child) = sysmon_child {
        let _ = child.kill();
    }
    println!("[e404-daemon] Daemon stopped cleanly.");
}

fn run_poll_fallback(cfg: &DaemonConfig, term: Arc<AtomicBool>) {
    let mut in_game = false;
    let mut in_battery_saver = false;
    let mut last_pkg = String::new();
    let mut last_normal_prof = "balanced".to_string();

    while !term.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(500));
        process_sysmon_event(&cfg, &mut in_game, &mut in_battery_saver, &mut last_pkg, &mut last_normal_prof);
    }
}

fn signal_hook_init(term: Arc<AtomicBool>) {
    unsafe {
        libc::signal(libc::SIGINT, sig_handler as usize);
        libc::signal(libc::SIGTERM, sig_handler as usize);
    }
}

extern "C" fn sig_handler(_: libc::c_int) {
    std::process::exit(0);
}
