import AppKit
import WebKit

final class AetherAppDelegate: NSObject, NSApplicationDelegate, NSWindowDelegate {
    var window: NSWindow!
    var webView: WKWebView!
    let targetUrlString: String

    init(url: String) {
        self.targetUrlString = url
        super.init()
    }

    func applicationDidFinishLaunching(_ notification: Notification) {
        setupMainMenu()

        let rect = NSRect(x: 0, y: 0, width: 1320, height: 860)
        let style: NSWindow.StyleMask = [
            .titled,
            .closable,
            .miniaturizable,
            .resizable,
            .fullSizeContentView
        ]

        window = NSWindow(
            contentRect: rect,
            styleMask: style,
            backing: .buffered,
            defer: false
        )
        window.title = "Aether (AuraMind) — AI Command Center & CRO Benchmark Studio"
        window.titlebarAppearsTransparent = true
        window.titleVisibility = .visible
        window.isMovableByWindowBackground = false
        window.appearance = NSAppearance(named: .darkAqua)
        // Obsidian Dark #070D14
        window.backgroundColor = NSColor(srgbRed: 7.0 / 255.0, green: 13.0 / 255.0, blue: 20.0 / 255.0, alpha: 0.96)
        window.minSize = NSSize(width: 960, height: 680)
        window.delegate = self
        window.center()

        let effectView = NSVisualEffectView(frame: rect)
        effectView.autoresizingMask = [.width, .height]
        effectView.material = .hudWindow
        effectView.blendingMode = .behindWindow
        effectView.state = .active

        let config = WKWebViewConfiguration()
        webView = WKWebView(frame: rect, configuration: config)
        webView.autoresizingMask = [.width, .height]
        webView.setValue(false, forKey: "drawsBackground")

        effectView.addSubview(webView)
        window.contentView = effectView

        if let url = URL(string: targetUrlString) {
            webView.load(URLRequest(url: url))
        }

        window.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
        return true
    }

    private func setupMainMenu() {
        let mainMenu = NSMenu()
        let appMenuItem = NSMenuItem()
        mainMenu.addItem(appMenuItem)

        let appMenu = NSMenu()
        let quitTitle = "Quit Aether"
        let quitItem = NSMenuItem(
            title: quitTitle,
            action: #selector(NSApplication.terminate(_:)),
            keyEquivalent: "q"
        )
        appMenu.addItem(quitItem)
        appMenuItem.submenu = appMenu
        NSApp.mainMenu = mainMenu
    }
}

let args = CommandLine.arguments
let urlArg = args.count > 1 ? args[1] : "http://127.0.0.1:4320"

let app = NSApplication.shared
app.setActivationPolicy(.regular)
let delegate = AetherAppDelegate(url: urlArg)
app.delegate = delegate
app.run()
