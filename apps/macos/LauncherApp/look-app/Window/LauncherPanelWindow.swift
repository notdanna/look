import AppKit

/// Borderless, because macOS clips a *titled* window to its own corner radius
/// whatever `layer.cornerRadius` says, so the panel could never go square.
/// Borderless refuses key status by default, hence the overrides.
final class LauncherPanelWindow: NSWindow {
    override var canBecomeKey: Bool { true }
    override var canBecomeMain: Bool { true }

    override func performKeyEquivalent(with event: NSEvent) -> Bool {
        let flags = event.modifierFlags.intersection(.deviceIndependentFlagsMask)

        // ⌘Q quits Look cleanly
        if flags == [.command],
           event.charactersIgnoringModifiers?.lowercased() == "q" || event.keyCode == 12
        {
            NSApplication.shared.terminate(nil)
            return true
        }

        // ⌘, or ⌘⇧, opens the config file directly
        if (flags == [.command] || flags == [.command, .shift]),
           event.charactersIgnoringModifiers == "," || event.keyCode == 43
        {
            ThemeStore.shared.openConfigFile()
            return true
        }

        return super.performKeyEquivalent(with: event)
    }
}
