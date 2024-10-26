# Space Monitor

Space Monitor is a Rust API for subscribing to real-time changes on Mac OS X to obtain the current active [space](https://support.apple.com/guide/mac-help/work-in-multiple-spaces-mh14112/mac) (virtual desktop) index.

Heavily inspired by the great work of [George Christou](https://github.com/gechr) and his Swift project - [WhichSpace](https://github.com/gechr/WhichSpace).

## Examples

Check usage in the [examples](./examples/) directory

## How it works

Surprisingly, obtaining the active virtual desktop index is a non-trivial task on Mac OS X and attempts in doing so have been breaking release after release as the method relies on undocumented Mac OS native APIs.

This method relies on a few key ingredients:

- Core Graphics (CG)

  - We use `CGSMainConnectionID` to get a connection to the main window server
  - The CGS (core graphics services) API is exploited to obtain this information

- FFI (Foreign function interface)

  - Bridge for us to call the C APIs from Rust

- Cocoa

  - Apple's native API for Mac OS apps
  - `NSApplication` for background app
  - Handle system notifications

- Objective-C
  - Some message-passing invocations (`msg_send!`)
  - Used for receiving event notifications

Space monitor is essentially a Rust binding to access lower-level mac OS internal APIs is an easy and efficient way.

While you can occassionally deciper some esoteric plist files to derive the active screen via `defaults read com.apple.spaces SpacesDisplayConfiguration`, the contents are almost always incorrect and out of date, which makes it a non-starter for realtime change detection.

## Warning

As this crate relies on private, undocumented native Mac OS APIs internally, I _believe_ your app would be rejected from the Apple app store if this crate is used within your application. However, users can still install the application externally.
