import Cocoa

class AppDelegate: NSObject, NSApplicationDelegate, NSMenuDelegate {
    let conn: CGConnectionID = CGSMainConnectionID()
    static var darkModeEnabled = false
    
    override init() {
        super.init()
        setupApplication()
        setupObservers()
        updateActiveSpaceNumber()
    }
    
    private func setupApplication() {
        NSApplication.shared.setActivationPolicy(.accessory)
        updateDarkModeStatus()
    }
    
    private func setupObservers() {
        NSWorkspace.shared.notificationCenter.addObserver(
            self,
            selector: #selector(updateActiveSpaceNumber),
            name: NSWorkspace.activeSpaceDidChangeNotification,
            object: NSWorkspace.shared
        )
        
        DistributedNotificationCenter.default().addObserver(
            self,
            selector: #selector(updateDarkModeStatus),
            name: NSNotification.Name("AppleInterfaceThemeChangedNotification"),
            object: nil
        )
        
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(updateActiveSpaceNumber),
            name: NSApplication.didUpdateNotification,
            object: nil
        )
    }
    
    @objc func updateDarkModeStatus(_ sender: AnyObject? = nil) {
        let dictionary = UserDefaults.standard.persistentDomain(forName: UserDefaults.globalDomain)
        if let interfaceStyle = dictionary?["AppleInterfaceStyle"] as? NSString {
            AppDelegate.darkModeEnabled = interfaceStyle.localizedCaseInsensitiveContains("dark")
        } else {
            AppDelegate.darkModeEnabled = false
        }
    }
    
    @objc func updateActiveSpaceNumber() {
        let displays = CGSCopyManagedDisplaySpaces(conn) as! [NSDictionary]
        let activeDisplay = CGSCopyActiveMenuBarDisplayIdentifier(conn) as! String
        let allSpaces: NSMutableArray = []
        var activeSpaceID = -1
        
        for d in displays {
            guard
                let current = d["Current Space"] as? [String: Any],
                let spaces = d["Spaces"] as? [[String: Any]],
                let dispID = d["Display Identifier"] as? String
            else {
                continue
            }
            
            switch dispID {
            case "Main", activeDisplay:
                activeSpaceID = current["ManagedSpaceID"] as! Int
            default:
                break
            }
            
            for s in spaces {
                let isFullscreen = s["TileLayoutManager"] as? [String: Any] != nil
                if isFullscreen {
                    continue
                }
                allSpaces.add(s)
            }
        }
        
        if activeSpaceID == -1 {
            print("No active space found.")
            return
        }
        
        for (index, space) in allSpaces.enumerated() {
            let spaceID = (space as! NSDictionary)["ManagedSpaceID"] as! Int
            let spaceNumber = index + 1
            if spaceID == activeSpaceID {
                print("Active Space Number: \(spaceNumber)")
                return
            }
        }
    }
    
    @objc func quitClicked(_ sender: NSMenuItem) {
        NSApplication.shared.terminate(self)
    }
}

