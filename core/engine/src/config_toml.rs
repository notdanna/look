use std::collections::HashMap;

/// Maps a structured TOML value tree into a flat map of legacy key-value strings,
/// allowing existing engine and frontend logic to consume values transparently.
pub fn toml_to_flat_map(toml_str: &str) -> Result<HashMap<String, String>, String> {
    let value: toml::Value = toml::from_str(toml_str).map_err(|e| format!("TOML syntax error: {e}"))?;
    let toml::Value::Table(root) = value else {
        return Err("Root of TOML config must be a table".to_string());
    };

    let mut out = HashMap::new();

    // [general]
    if let Some(toml::Value::Table(general)) = root.get("general") {
        if let Some(v) = general.get("launcher_hotkey").and_then(val_to_str) {
            out.insert("launcher_hotkey".into(), v);
        }
        if let Some(v) = general.get("launch_at_login").and_then(val_to_str) {
            out.insert("launch_at_login".into(), v);
        }
        if let Some(v) = general.get("animations").and_then(val_to_str) {
            out.insert("animations_enabled".into(), v);
        }
        if let Some(v) = general.get("query_retention_seconds").and_then(val_to_str) {
            out.insert("query_retention_seconds".into(), v);
        }
        if let Some(v) = general.get("log_level").and_then(val_to_str) {
            out.insert("backend_log_level".into(), v);
        }
    }

    // [appearance]
    if let Some(toml::Value::Table(app)) = root.get("appearance") {
        if let Some(v) = app.get("theme").and_then(val_to_str) {
            out.insert("ui_theme".into(), v);
        }
        if let Some(v) = app.get("layout").and_then(val_to_str) {
            out.insert("layout".into(), v);
        }
        if let Some(v) = app.get("window_width").and_then(val_to_str) {
            out.insert("window_width".into(), v);
        }
        if let Some(v) = app.get("search_bar_width").and_then(val_to_str) {
            out.insert("search_bar_width".into(), v);
        }
        if let Some(v) = app.get("search_bar_height").and_then(val_to_str) {
            out.insert("search_bar_height".into(), v);
        }
        if let Some(v) = app.get("inner_gap").and_then(val_to_str) {
            out.insert("inner_gap".into(), v);
        }
        if let Some(v) = app.get("corner_radius").and_then(val_to_str) {
            out.insert("ui_surface_radius".into(), v);
        }

        // [appearance.tint]
        if let Some(toml::Value::Table(tint)) = app.get("tint") {
            if let Some(color) = tint.get("color").and_then(val_to_f64_array) {
                if color.len() >= 4 {
                    out.insert("ui_tint_red".into(), format!("{:.2}", color[0]));
                    out.insert("ui_tint_green".into(), format!("{:.2}", color[1]));
                    out.insert("ui_tint_blue".into(), format!("{:.2}", color[2]));
                    out.insert("ui_tint_opacity".into(), format!("{:.2}", color[3]));
                }
            }
        }

        // [appearance.blur]
        if let Some(toml::Value::Table(blur)) = app.get("blur") {
            if let Some(v) = blur.get("material").and_then(val_to_str) {
                out.insert("ui_blur_material".into(), v);
            }
            if let Some(v) = blur.get("opacity").and_then(val_to_str) {
                out.insert("ui_blur_opacity".into(), v);
            }
        }

        // [appearance.font]
        if let Some(toml::Value::Table(font)) = app.get("font") {
            if let Some(v) = font.get("name").and_then(val_to_str) {
                out.insert("ui_font_name".into(), v);
            }
            if let Some(v) = font.get("size").and_then(val_to_str) {
                out.insert("ui_font_size".into(), v);
            }
            if let Some(color) = font.get("color").and_then(val_to_f64_array) {
                if color.len() >= 4 {
                    out.insert("ui_font_red".into(), format!("{:.2}", color[0]));
                    out.insert("ui_font_green".into(), format!("{:.2}", color[1]));
                    out.insert("ui_font_blue".into(), format!("{:.2}", color[2]));
                    out.insert("ui_font_opacity".into(), format!("{:.2}", color[3]));
                }
            }
        }

        // [appearance.border]
        if let Some(toml::Value::Table(border)) = app.get("border") {
            if let Some(v) = border.get("thickness").and_then(val_to_str) {
                out.insert("ui_border_thickness".into(), v);
            }
            if let Some(color) = border.get("color").and_then(val_to_f64_array) {
                if color.len() >= 4 {
                    out.insert("ui_border_red".into(), format!("{:.2}", color[0]));
                    out.insert("ui_border_green".into(), format!("{:.2}", color[1]));
                    out.insert("ui_border_blue".into(), format!("{:.2}", color[2]));
                    out.insert("ui_border_opacity".into(), format!("{:.2}", color[3]));
                }
            }
        }

        // [appearance.background_image]
        if let Some(toml::Value::Table(bg)) = app.get("background_image") {
            if let Some(v) = bg.get("path").and_then(val_to_str) {
                out.insert("ui_background_image".into(), v);
            }
            if let Some(v) = bg.get("mode").and_then(val_to_str) {
                out.insert("ui_background_image_mode".into(), v);
            }
            if let Some(v) = bg.get("opacity").and_then(val_to_str) {
                out.insert("ui_background_image_opacity".into(), v);
            }
            if let Some(v) = bg.get("blur").and_then(val_to_str) {
                out.insert("ui_background_image_blur".into(), v);
            }
        }
    }

    // [running_apps]
    if let Some(toml::Value::Table(ra)) = root.get("running_apps") {
        if let Some(v) = ra.get("placement").and_then(val_to_str) {
            out.insert("running_apps_placement".into(), v);
        }
        if let Some(v) = ra.get("theme_tint").and_then(val_to_str) {
            out.insert("running_apps_theme_tint".into(), v);
        }
    }

    // [launchpad]
    if let Some(toml::Value::Table(lp)) = root.get("launchpad") {
        if let Some(v) = lp.get("enabled").and_then(val_to_str) {
            out.insert("super_actions_enabled".into(), v);
        }
    }

    // [ai]
    if let Some(toml::Value::Table(ai)) = root.get("ai") {
        if let Some(v) = ai.get("enabled").and_then(val_to_str) {
            out.insert("ai_enabled".into(), v);
        }
        if let Some(v) = ai.get("provider").and_then(val_to_str) {
            out.insert("ai_provider".into(), v);
        }
        if let Some(v) = ai.get("allow_remote_context").and_then(val_to_str) {
            out.insert("ai_allow_remote_context".into(), v);
        }
        // [ai.ollama]
        if let Some(toml::Value::Table(ollama)) = ai.get("ollama") {
            if let Some(v) = ollama.get("host").and_then(val_to_str) {
                out.insert("ollama_host".into(), v);
            }
            if let Some(v) = ollama.get("model").and_then(val_to_str) {
                out.insert("ollama_model".into(), v);
            }
        }
    }

    // [clipboard]
    if let Some(toml::Value::Table(clip)) = root.get("clipboard") {
        if let Some(v) = clip.get("history_limit").and_then(val_to_str) {
            out.insert("clipboard_history_limit".into(), v);
        }
        if let Some(v) = clip.get("image_limit").and_then(val_to_str) {
            out.insert("clipboard_image_limit".into(), v);
        }
    }

    // [indexing]
    if let Some(toml::Value::Table(idx)) = root.get("indexing") {
        if let Some(v) = idx.get("lazy").and_then(val_to_str) {
            out.insert("lazy_indexing_enabled".into(), v);
        }
        if let Some(v) = idx.get("localized_app_names").and_then(val_to_str) {
            out.insert("localized_app_names".into(), v);
        }
        if let Some(v) = idx.get("skip_dir_names").and_then(val_to_csv) {
            out.insert("skip_dir_names".into(), v);
        }

        // [indexing.apps]
        if let Some(toml::Value::Table(apps)) = idx.get("apps") {
            if let Some(v) = apps.get("roots").and_then(val_to_csv) {
                out.insert("app_scan_roots".into(), v);
            }
            if let Some(v) = apps.get("depth").and_then(val_to_str) {
                out.insert("app_scan_depth".into(), v);
            }
            if let Some(v) = apps.get("exclude_paths").and_then(val_to_csv) {
                out.insert("app_exclude_paths".into(), v);
            }
            if let Some(v) = apps.get("exclude_names").and_then(val_to_csv) {
                out.insert("app_exclude_names".into(), v);
            }
        }

        // [indexing.files]
        if let Some(toml::Value::Table(files)) = idx.get("files") {
            if let Some(v) = files.get("roots").and_then(val_to_csv) {
                out.insert("file_scan_roots".into(), v);
            }
            if let Some(v) = files.get("extra_roots").and_then(val_to_csv) {
                out.insert("file_scan_extra_roots".into(), v);
            }
            if let Some(v) = files.get("depth").and_then(val_to_str) {
                out.insert("file_scan_depth".into(), v);
            }
            if let Some(v) = files.get("limit").and_then(val_to_str) {
                out.insert("file_scan_limit".into(), v);
            }
            if let Some(v) = files.get("exclude_paths").and_then(val_to_csv) {
                out.insert("file_exclude_paths".into(), v);
            }
        }

        // [indexing.ignored_patterns]
        if let Some(toml::Value::Table(patterns)) = idx.get("ignored_patterns") {
            for (grp, val) in patterns {
                if let Some(csv_or_pipe) = val_to_pipe_separated(val) {
                    out.insert(format!("ignored_patterns_{grp}"), csv_or_pipe);
                }
            }
        }
    }

    // [tools]
    if let Some(toml::Value::Table(tools)) = root.get("tools") {
        for (k, v) in tools {
            if let Some(s) = val_to_str(v) {
                out.insert(k.clone(), s);
            }
        }
    }

    // [aliases]
    if let Some(toml::Value::Table(aliases)) = root.get("aliases") {
        for (k, v) in aliases {
            if let Some(pipes) = val_to_pipe_separated(v) {
                out.insert(format!("alias_{k}"), pipes);
            }
        }
    }

    // [pomo]
    if let Some(toml::Value::Table(pomo)) = root.get("pomo") {
        if let Some(v) = pomo.get("sessions").and_then(val_to_str) {
            out.insert("pomo_sessions".into(), v);
        }
        if let Some(v) = pomo.get("timer_style").and_then(val_to_str) {
            out.insert("pomo_timer_style".into(), v);
        }
        if let Some(v) = pomo.get("music_folder").and_then(val_to_str) {
            out.insert("pomo_music_folder".into(), v);
        }
    }

    // [shortcuts]
    if let Some(toml::Value::Table(sc)) = root.get("shortcuts") {
        for (k, v) in sc {
            if let Some(s) = val_to_str(v) {
                out.insert(k.clone(), s);
            }
        }
    }

    Ok(out)
}

fn val_to_str(val: &toml::Value) -> Option<String> {
    match val {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Integer(i) => Some(i.to_string()),
        toml::Value::Float(f) => {
            if f.fract() == 0.0 {
                Some(format!("{f:.1}"))
            } else {
                Some(f.to_string())
            }
        }
        toml::Value::Boolean(b) => Some(b.to_string()),
        _ => None,
    }
}

fn val_to_f64_array(val: &toml::Value) -> Option<Vec<f64>> {
    let toml::Value::Array(arr) = val else { return None };
    let mut result = Vec::new();
    for item in arr {
        match item {
            toml::Value::Float(f) => result.push(*f),
            toml::Value::Integer(i) => result.push(*i as f64),
            _ => return None,
        }
    }
    Some(result)
}

fn val_to_csv(val: &toml::Value) -> Option<String> {
    match val {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().filter_map(val_to_str).collect();
            Some(items.join(","))
        }
        _ => None,
    }
}

fn val_to_pipe_separated(val: &toml::Value) -> Option<String> {
    match val {
        toml::Value::String(s) => Some(s.clone()),
        toml::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().filter_map(val_to_str).collect();
            Some(items.join("|"))
        }
        _ => None,
    }
}

/// Generates default config TOML contents.
pub fn default_toml_contents() -> String {
    r#"# Look configuration
# Generated on first launch. Edit values, then reload with Cmd+Shift+R or Cmd+Shift+;
# (Ctrl+Shift+; on Linux and Windows). Press Cmd+, to open this config in your editor.

[general]
launcher_hotkey = "cmd+space"
launch_at_login = true
animations = true
query_retention_seconds = 5
log_level = "error"

[appearance]
theme = ""
layout = "split"
window_width = 860
search_bar_width = 860
search_bar_height = 45
inner_gap = 7
corner_radius = 1.5

[appearance.tint]
# Red, Green, Blue, Opacity (0.0 - 1.0)
color = [0.08, 0.10, 0.12, 0.55]

[appearance.blur]
material = "hudWindow"
opacity = 0.95

[appearance.font]
name = "SF Pro Text"
size = 14
color = [0.96, 0.96, 0.98, 0.96]

[appearance.border]
thickness = 1.0
color = [1.0, 1.0, 1.0, 0.12]

[appearance.background_image]
# path = "~/Pictures/wallpaper.png"
# mode = "fill"
# opacity = 1.0
# blur = 0.0

[running_apps]
placement = "right"
theme_tint = true

[launchpad]
enabled = false

[ai]
enabled = true
provider = "appleIntelligence"
allow_remote_context = false

[ai.ollama]
host = "http://localhost:11434"
model = "qwen3.5:4b"

[clipboard]
history_limit = 10
image_limit = 10

[indexing]
lazy = true
localized_app_names = false
skip_dir_names = ["node_modules", "target", "build", "dist", "library", "applications", "old firefox data", "deriveddata", "pods", "vendor", "out", "coverage", "tmp", "cache", "venv"]

[indexing.apps]
roots = ["/Applications", "/System/Applications", "/System/Applications/Utilities", "/System/Library/CoreServices/Applications", "/System/Library/CoreServices/Finder.app/Contents/Applications"]
depth = 3
exclude_paths = []
exclude_names = []

[indexing.files]
roots = ["Desktop", "Documents", "Downloads"]
extra_roots = []
depth = 4
limit = 8000
exclude_paths = []

[indexing.ignored_patterns]
# browser = ["~/AppData/Local/BraveSoftware/**/*.log", "~/AppData/Local/Google/Chrome/**/*.tmp"]

[tools]
# text_editor = "nvim"
# code_editor = "zed"
# terminal = "ghostty"

[aliases]
note = ["Notion", "Obsidian", "Notes", "Apple Notes", "Bear", "Logseq"]
code = ["Visual Studio Code", "VSCode", "Cursor", "Windsurf", "IntelliJ IDEA", "PyCharm", "WebStorm", "Neovim", "Xcode", "Zed"]
term = ["Terminal", "iTerm", "iTerm2", "Ghostty", "WezTerm", "Alacritty", "Kitty", "Warp"]
chat = ["Slack", "Discord", "Telegram", "Messages"]
music = ["Spotify", "Apple Music", "Music"]
brow = ["Safari", "Arc", "Google Chrome", "Chrome", "Firefox", "Brave"]

[pomo]
timer_style = "modern"
"#
    .to_string()
}

/// Converts a legacy flat key=value config into categorized TOML.
pub fn migrate_legacy_flat_to_toml(flat_contents: &str) -> String {
    let mut kvs: HashMap<String, String> = HashMap::new();
    for line in flat_contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once('=') {
            let k = k.trim();
            let v = v.trim();
            if !k.is_empty() {
                kvs.insert(k.to_string(), v.to_string());
            }
        }
    }

    let mut doc = default_toml_contents();

    // Map each existing key into TOML
    for (k, v) in &kvs {
        match k.as_str() {
            "launcher_hotkey" => set_toml_value(&mut doc, "general", "launcher_hotkey", &format!("\"{v}\"")),
            "launch_at_login" => set_toml_value(&mut doc, "general", "launch_at_login", v),
            "animations_enabled" => set_toml_value(&mut doc, "general", "animations", v),
            "query_retention_seconds" => set_toml_value(&mut doc, "general", "query_retention_seconds", v),
            "backend_log_level" => set_toml_value(&mut doc, "general", "log_level", &format!("\"{v}\"")),

            "ui_theme" => set_toml_value(&mut doc, "appearance", "theme", &format!("\"{v}\"")),
            "layout" => set_toml_value(&mut doc, "appearance", "layout", &format!("\"{v}\"")),
            "window_width" | "content_width" => set_toml_value(&mut doc, "appearance", "window_width", v),
            "search_bar_width" | "bar_width" => set_toml_value(&mut doc, "appearance", "search_bar_width", v),
            "search_bar_height" | "bar_height" => set_toml_value(&mut doc, "appearance", "search_bar_height", v),
            "inner_gap" => set_toml_value(&mut doc, "appearance", "inner_gap", v),
            "ui_surface_radius" => set_toml_value(&mut doc, "appearance", "corner_radius", v),

            "ui_blur_material" => set_toml_value(&mut doc, "appearance.blur", "material", &format!("\"{v}\"")),
            "ui_blur_opacity" => set_toml_value(&mut doc, "appearance.blur", "opacity", v),

            "ui_font_name" => set_toml_value(&mut doc, "appearance.font", "name", &format!("\"{v}\"")),
            "ui_font_size" => set_toml_value(&mut doc, "appearance.font", "size", v),

            "ui_border_thickness" | "ui_border_width" | "border_width" | "border_thickness" => {
                set_toml_value(&mut doc, "appearance.border", "thickness", v)
            }

            "ui_background_image" => set_toml_value(&mut doc, "appearance.background_image", "path", &format!("\"{v}\"")),
            "ui_background_image_mode" => set_toml_value(&mut doc, "appearance.background_image", "mode", &format!("\"{v}\"")),
            "ui_background_image_opacity" => set_toml_value(&mut doc, "appearance.background_image", "opacity", v),
            "ui_background_image_blur" => set_toml_value(&mut doc, "appearance.background_image", "blur", v),

            "running_apps_placement" => set_toml_value(&mut doc, "running_apps", "placement", &format!("\"{v}\"")),
            "running_apps_theme_tint" => set_toml_value(&mut doc, "running_apps", "theme_tint", v),

            "super_actions_enabled" => set_toml_value(&mut doc, "launchpad", "enabled", v),

            "ai_enabled" => set_toml_value(&mut doc, "ai", "enabled", v),
            "ai_provider" => set_toml_value(&mut doc, "ai", "provider", &format!("\"{v}\"")),
            "ai_allow_remote_context" => set_toml_value(&mut doc, "ai", "allow_remote_context", v),
            "ollama_host" => set_toml_value(&mut doc, "ai.ollama", "host", &format!("\"{v}\"")),
            "ollama_model" => set_toml_value(&mut doc, "ai.ollama", "model", &format!("\"{v}\"")),

            "clipboard_history_limit" => set_toml_value(&mut doc, "clipboard", "history_limit", v),
            "clipboard_image_limit" => set_toml_value(&mut doc, "clipboard", "image_limit", v),

            "lazy_indexing_enabled" => set_toml_value(&mut doc, "indexing", "lazy", v),
            "localized_app_names" => set_toml_value(&mut doc, "indexing", "localized_app_names", v),
            "skip_dir_names" => {
                let items: Vec<String> = v.split(',').map(|s| format!("\"{}\"", s.trim())).collect();
                set_toml_value(&mut doc, "indexing", "skip_dir_names", &format!("[{}]", items.join(", ")));
            }
            "app_scan_depth" => set_toml_value(&mut doc, "indexing.apps", "depth", v),
            "file_scan_depth" => set_toml_value(&mut doc, "indexing.files", "depth", v),
            "file_scan_limit" => set_toml_value(&mut doc, "indexing.files", "limit", v),

            "pomo_sessions" => set_toml_value(&mut doc, "pomo", "sessions", &format!("\"{v}\"")),
            "pomo_timer_style" => set_toml_value(&mut doc, "pomo", "timer_style", &format!("\"{v}\"")),
            "pomo_music_folder" => set_toml_value(&mut doc, "pomo", "music_folder", &format!("\"{v}\"")),

            _ => {}
        }
    }

    // Handle tint color array if red/green/blue/opacity present
    let tr = kvs.get("ui_tint_red").map(|s| s.as_str()).unwrap_or("0.08");
    let tg = kvs.get("ui_tint_green").map(|s| s.as_str()).unwrap_or("0.10");
    let tb = kvs.get("ui_tint_blue").map(|s| s.as_str()).unwrap_or("0.12");
    let to = kvs.get("ui_tint_opacity").map(|s| s.as_str()).unwrap_or("0.55");
    set_toml_value(&mut doc, "appearance.tint", "color", &format!("[{tr}, {tg}, {tb}, {to}]"));

    // Handle font color array
    let fr = kvs.get("ui_font_red").map(|s| s.as_str()).unwrap_or("0.96");
    let fg = kvs.get("ui_font_green").map(|s| s.as_str()).unwrap_or("0.96");
    let fb = kvs.get("ui_font_blue").map(|s| s.as_str()).unwrap_or("0.98");
    let fo = kvs.get("ui_font_opacity").map(|s| s.as_str()).unwrap_or("0.96");
    set_toml_value(&mut doc, "appearance.font", "color", &format!("[{fr}, {fg}, {fb}, {fo}]"));

    // Handle border color array
    let br = kvs.get("ui_border_red").or_else(|| kvs.get("border_red")).map(|s| s.as_str()).unwrap_or("1.0");
    let bg = kvs.get("ui_border_green").or_else(|| kvs.get("border_green")).map(|s| s.as_str()).unwrap_or("1.0");
    let bb = kvs.get("ui_border_blue").or_else(|| kvs.get("border_blue")).map(|s| s.as_str()).unwrap_or("1.0");
    let bo = kvs.get("ui_border_opacity").or_else(|| kvs.get("border_opacity")).map(|s| s.as_str()).unwrap_or("0.12");
    set_toml_value(&mut doc, "appearance.border", "color", &format!("[{br}, {bg}, {bb}, {bo}]"));

    // Array migrations
    if let Some(app_roots) = kvs.get("app_scan_roots") {
        let items: Vec<String> = app_roots.split(',').map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.apps", "roots", &format!("[{}]", items.join(", ")));
    }
    if let Some(app_ex_names) = kvs.get("app_exclude_names") {
        let items: Vec<String> = app_ex_names.split(',').filter(|s| !s.trim().is_empty()).map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.apps", "exclude_names", &format!("[{}]", items.join(", ")));
    }
    if let Some(app_ex_paths) = kvs.get("app_exclude_paths") {
        let items: Vec<String> = app_ex_paths.split(',').filter(|s| !s.trim().is_empty()).map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.apps", "exclude_paths", &format!("[{}]", items.join(", ")));
    }
    if let Some(file_roots) = kvs.get("file_scan_roots") {
        let items: Vec<String> = file_roots.split(',').map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.files", "roots", &format!("[{}]", items.join(", ")));
    }
    if let Some(file_ex_roots) = kvs.get("file_scan_extra_roots") {
        let items: Vec<String> = file_ex_roots.split(',').filter(|s| !s.trim().is_empty()).map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.files", "extra_roots", &format!("[{}]", items.join(", ")));
    }
    if let Some(file_ex_paths) = kvs.get("file_exclude_paths") {
        let items: Vec<String> = file_ex_paths.split(',').filter(|s| !s.trim().is_empty()).map(|s| format!("\"{}\"", s.trim())).collect();
        set_toml_value(&mut doc, "indexing.files", "exclude_paths", &format!("[{}]", items.join(", ")));
    }

    doc
}

pub fn format_toml_value(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return "\"\"".to_string();
    }
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_string();
    }
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return trimmed.to_string();
    }
    if (trimmed.starts_with('"') && trimmed.ends_with('"')) || (trimmed.starts_with('\'') && trimmed.ends_with('\'')) {
        return trimmed.to_string();
    }
    if trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok() {
        return trimmed.to_string();
    }
    format!("\"{}\"", trimmed.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Updates or inserts a `key = value` line within a specific `[section]` table in a TOML string,
/// preserving existing structure and comments.
pub fn set_toml_value(doc: &mut String, section: &str, key: &str, raw_val: &str) {
    let formatted_val = format_toml_value(raw_val);
    let header = format!("[{section}]");
    let mut lines: Vec<String> = doc.lines().map(String::from).collect();
    let mut in_section = false;
    let mut key_updated = false;
    let mut section_start_idx = None;
    let mut next_section_idx = None;

    let mut found_key: Option<(usize, bool)> = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == header {
            in_section = true;
            section_start_idx = Some(i);
            continue;
        } else if in_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            next_section_idx = Some(i);
            break;
        }

        if in_section {
            if let Some((k, old_val)) = trimmed.split_once('=') {
                if k.trim() == key {
                    let is_multiline_start = old_val.contains('[') && !old_val.contains(']');
                    found_key = Some((i, is_multiline_start));
                    break;
                }
            }
        }
    }

    if let Some((idx, is_multiline)) = found_key {
        lines[idx] = format!("{key} = {formatted_val}");
        key_updated = true;
        if is_multiline {
            let j = idx + 1;
            while j < lines.len() {
                let next_trimmed = lines[j].trim();
                if next_trimmed.starts_with('[') && next_trimmed.ends_with(']') {
                    break;
                }
                let has_closing = next_trimmed.contains(']');
                lines.remove(j);
                if has_closing {
                    break;
                }
            }
        }
    }

    if !key_updated {
        if in_section {
            let insert_at = next_section_idx.unwrap_or(lines.len());
            lines.insert(insert_at, format!("{key} = {formatted_val}"));
        } else if let Some(_sec_idx) = section_start_idx {
            let insert_at = next_section_idx.unwrap_or(lines.len());
            lines.insert(insert_at, format!("{key} = {formatted_val}"));
        } else {
            // Section doesn't exist, append it at the end
            lines.push(String::new());
            lines.push(header);
            lines.push(format!("{key} = {formatted_val}"));
        }
    }

    *doc = lines.join("\n");
    if !doc.ends_with('\n') {
        doc.push('\n');
    }
}

/// Removes a key from a section in TOML if present.
pub fn remove_toml_value(doc: &mut String, section: &str, key: &str) {
    let header = format!("[{section}]");
    let mut lines: Vec<String> = doc.lines().map(String::from).collect();
    let mut in_section = false;
    let mut remove_idx = None;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == header {
            in_section = true;
            continue;
        } else if in_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            break;
        }

        if in_section {
            if let Some((k, _)) = trimmed.split_once('=') {
                if k.trim() == key {
                    remove_idx = Some(i);
                    break;
                }
            }
        }
    }

    if let Some(idx) = remove_idx {
        lines.remove(idx);
        *doc = lines.join("\n");
        if !doc.ends_with('\n') {
            doc.push('\n');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_toml_roundtrip() {
        let toml_str = default_toml_contents();
        let map = toml_to_flat_map(&toml_str).expect("parse default toml");

        assert_eq!(map.get("launcher_hotkey").unwrap(), "cmd+space");
        assert_eq!(map.get("launch_at_login").unwrap(), "true");
        assert_eq!(map.get("animations_enabled").unwrap(), "true");
        assert_eq!(map.get("window_width").unwrap(), "860");
        assert_eq!(map.get("search_bar_width").unwrap(), "860");
        assert_eq!(map.get("ui_surface_radius").unwrap(), "1.5");
        assert_eq!(map.get("ui_tint_red").unwrap(), "0.08");
        assert_eq!(map.get("ui_tint_green").unwrap(), "0.10");
        assert_eq!(map.get("ui_tint_blue").unwrap(), "0.12");
        assert_eq!(map.get("ui_tint_opacity").unwrap(), "0.55");
        assert_eq!(map.get("ui_font_name").unwrap(), "SF Pro Text");
        assert_eq!(map.get("ui_font_size").unwrap(), "14");
        assert_eq!(map.get("ui_font_red").unwrap(), "0.96");
        assert_eq!(map.get("ui_border_thickness").unwrap(), "1.0");
        assert_eq!(map.get("ui_border_opacity").unwrap(), "0.12");
        assert_eq!(map.get("running_apps_placement").unwrap(), "right");
        assert_eq!(map.get("ai_provider").unwrap(), "appleIntelligence");
        assert_eq!(map.get("ollama_host").unwrap(), "http://localhost:11434");
        assert_eq!(map.get("ollama_model").unwrap(), "qwen3.5:4b");
        assert_eq!(map.get("clipboard_history_limit").unwrap(), "10");
        assert_eq!(map.get("lazy_indexing_enabled").unwrap(), "true");
        assert!(map.get("app_scan_roots").unwrap().contains("/Applications"));
        assert!(map.get("alias_note").unwrap().contains("Notion|Obsidian"));
    }

    #[test]
    fn migrates_legacy_flat_config() {
        let flat = "ui_theme=dracula\nwindow_width=920\nui_tint_red=0.2\napp_exclude_names=Calculator,Mail\n";
        let toml_str = migrate_legacy_flat_to_toml(flat);
        let map = toml_to_flat_map(&toml_str).expect("parse migrated toml");

        assert_eq!(map.get("ui_theme").unwrap(), "dracula");
        assert_eq!(map.get("window_width").unwrap(), "920");
        assert_eq!(map.get("ui_tint_red").unwrap(), "0.20");
        assert_eq!(map.get("app_exclude_names").unwrap(), "Calculator,Mail");
    }

    #[test]
    fn updates_toml_preserving_lines() {
        let mut doc = default_toml_contents();
        set_toml_value(&mut doc, "appearance", "window_width", "1024");
        set_toml_value(&mut doc, "pomo", "music_folder", "\"/Users/test/Music\"");

        let map = toml_to_flat_map(&doc).expect("parse modified");
        assert_eq!(map.get("window_width").unwrap(), "1024");
        assert_eq!(map.get("pomo_music_folder").unwrap(), "/Users/test/Music");

        remove_toml_value(&mut doc, "pomo", "music_folder");
        let map2 = toml_to_flat_map(&doc).expect("parse after remove");
        assert_eq!(map2.get("pomo_music_folder"), None);
    }
}
