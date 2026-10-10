import Darwin
import Foundation

@_silgen_name("look_config_path")
nonisolated
private func look_config_path(_ dev: Bool) -> UnsafeMutablePointer<CChar>?

@_silgen_name("look_free_cstring")
nonisolated
private func look_free_config_cstring(_ ptr: UnsafeMutablePointer<CChar>?)

@_silgen_name("look_config_values_json")
nonisolated
private func look_config_values_json() -> UnsafeMutablePointer<CChar>?

@_silgen_name("look_config_set_value")
nonisolated
private func look_config_set_value(
    _ section: UnsafePointer<CChar>?,
    _ key: UnsafePointer<CChar>?,
    _ rawVal: UnsafePointer<CChar>?
) -> Bool

@_silgen_name("look_config_remove_value")
nonisolated
private func look_config_remove_value(
    _ section: UnsafePointer<CChar>?,
    _ key: UnsafePointer<CChar>?
) -> Bool

enum ConfigPathResolver {
    private static let productionBundleIdentifier = "noah-code.Look"

    /// Where this build reads and writes its settings.
    ///
    /// The app decides only WHICH build it is; the core decides where that
    /// build's file lives, including the one-time move of a legacy
    /// `~/.look.config` into `~/.look/`. Two implementations of "where is the
    /// config" could disagree, and the app would then read one file and save to
    /// another, which looks exactly like settings not persisting.
    static func resolvedPath() -> String {
        let env = ProcessInfo.processInfo.environment
        if let custom = env["LOOK_CONFIG_PATH"]?.trimmingCharacters(in: .whitespacesAndNewlines),
            !custom.isEmpty
        {
            // Ensured like every answer the core gives: an override into a
            // folder that does not exist yet is a path nothing can write.
            try? FileManager.default.createDirectory(
                atPath: (custom as NSString).deletingLastPathComponent,
                withIntermediateDirectories: true)
            return custom
        }

        let home = env["HOME"] ?? NSHomeDirectory()
        guard let ptr = look_config_path(isDevBuild) else {
            // The core could not answer (no HOME). Fall back to the legacy path
            // rather than losing the user's settings.
            return (home as NSString).appendingPathComponent(
                isDevBuild ? ".look/config.dev" : ".look.config")
        }
        defer { look_free_config_cstring(ptr) }
        return String(cString: ptr)
    }

    /// A dev build keeps its own settings, so running it never edits the
    /// installed copy's config.
    private static var isDevBuild: Bool {
        if let bundleIdentifier = Bundle.main.bundleIdentifier,
            bundleIdentifier.caseInsensitiveCompare(productionBundleIdentifier) != .orderedSame
        {
            return true
        }
        return Bundle.main.bundleURL
            .resolvingSymlinksInPath()
            .path
            .lowercased()
            .contains("/look dev.app")
    }

    static func applyDefaultConfigEnvironmentIfNeeded() {
        let env = ProcessInfo.processInfo.environment
        if let existing = env["LOOK_CONFIG_PATH"]?.trimmingCharacters(in: .whitespacesAndNewlines), !existing.isEmpty {
            return
        }

        let resolved = resolvedPath()
        setenv("LOOK_CONFIG_PATH", resolved, 1)
    }

    /// Loads the active configuration as a flat [key: value] map parsed by the Rust engine.
    /// If there is a parse error, it's captured in `["_error": message]`.
    static func loadFlatConfig() -> [String: String] {
        guard let ptr = look_config_values_json() else {
            return [:]
        }
        defer { look_free_config_cstring(ptr) }
        let jsonStr = String(cString: ptr)
        guard let data = jsonStr.data(using: .utf8),
              let dict = try? JSONSerialization.jsonObject(with: data) as? [String: String] else {
            return [:]
        }
        return dict
    }

    /// Sets a value in a section of the active configuration file.
    @discardableResult
    static func setConfigValue(section: String, key: String, rawValue: String) -> Bool {
        section.withCString { secPtr in
            key.withCString { keyPtr in
                rawValue.withCString { valPtr in
                    look_config_set_value(secPtr, keyPtr, valPtr)
                }
            }
        }
    }

    /// Removes a key from a section of the active configuration file.
    @discardableResult
    static func removeConfigValue(section: String, key: String) -> Bool {
        section.withCString { secPtr in
            key.withCString { keyPtr in
                look_config_remove_value(secPtr, keyPtr)
            }
        }
    }
}
