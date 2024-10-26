#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cocoa::appkit::NSApplicationActivationPolicy;
use cocoa::base::{id, YES};
use core_foundation::array::CFArray;
use core_foundation::string::CFString;
use objc::runtime::{Object, BOOL};
use objc::{class, msg_send, sel, sel_impl};
use std::cell::Cell;
use std::os::raw::{c_uint, c_void};
use std::sync::mpsc::{self, Receiver, Sender};

pub type CGConnectionID = c_uint;

#[derive(Debug, Clone)]
pub enum MonitorEvent {
    SpaceChange(i32),
}

pub struct SpaceMonitor {
    event_tx: Sender<MonitorEvent>,
}

impl SpaceMonitor {
    pub fn new() -> (Self, Receiver<MonitorEvent>) {
        let (tx, rx) = mpsc::channel();
        (SpaceMonitor { event_tx: tx }, rx)
    }

    pub fn start_listening(self) {
        println!("Starting space monitor...");
        println!("Press Ctrl+C to exit");
        AppDelegate::new(self.event_tx).start_listening();
    }

    pub fn get_current_space_number() -> i32 {
        unsafe {
            let conn = CGSMainConnectionID();
            let displays = CGSCopyManagedDisplaySpaces(conn);
            let active_display = CGSCopyActiveMenuBarDisplayIdentifier(conn);
            let displays_array: id = msg_send![class!(NSArray), arrayWithArray:displays];

            let mut active_space_id = -1;
            let mut all_spaces = Vec::new();

            let count: usize = msg_send![displays_array, count];
            for i in 0..count {
                let display: id = msg_send![displays_array, objectAtIndex:i];

                let current_space = make_nsstring("Current Space");
                let spaces = make_nsstring("Spaces");
                let disp_id = make_nsstring("Display Identifier");

                let current: id = msg_send![display, objectForKey:current_space];
                let spaces_arr: id = msg_send![display, objectForKey:spaces];
                let disp_identifier: id = msg_send![display, objectForKey:disp_id];

                if current.is_null() || spaces_arr.is_null() || disp_identifier.is_null() {
                    continue;
                }

                let disp_str: id = msg_send![disp_identifier, description];
                let main_str = make_nsstring("Main");
                let active_str = make_nsstring(&active_display.to_string());
                let is_main: BOOL = msg_send![disp_str, isEqualToString:main_str];
                let is_active: BOOL = msg_send![disp_str, isEqualToString:active_str];

                if is_main == YES || is_active == YES {
                    let space_id_key = make_nsstring("ManagedSpaceID");
                    active_space_id = msg_send![current, objectForKey:space_id_key];
                }

                let spaces_count: usize = msg_send![spaces_arr, count];
                for j in 0..spaces_count {
                    let space: id = msg_send![spaces_arr, objectAtIndex:j];
                    let tile_key = make_nsstring("TileLayoutManager");
                    let tile_layout: id = msg_send![space, objectForKey:tile_key];

                    if tile_layout.is_null() {
                        all_spaces.push(space);
                    }
                }
            }

            if active_space_id == -1 {
                return -1;
            }

            for (index, space) in all_spaces.iter().enumerate() {
                let space_id_key = make_nsstring("ManagedSpaceID");
                let space_id: i32 = msg_send![*space, objectForKey:space_id_key];
                let space_number = index + 1;

                if space_id == active_space_id {
                    return space_number as i32;
                }
            }
            -1
        }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGSMainConnectionID() -> CGConnectionID;
    fn CGSCopyManagedDisplaySpaces(connection: CGConnectionID) -> CFArray;
    fn CGSCopyActiveMenuBarDisplayIdentifier(connection: CGConnectionID) -> CFString;
}

unsafe fn make_nsstring(string: &str) -> id {
    let cls = class!(NSString);
    let string = std::ffi::CString::new(string).unwrap();
    msg_send![cls, stringWithUTF8String:string.as_ptr()]
}

struct AppState {
    conn: CGConnectionID,
    current_space: Cell<i32>,
    event_tx: Sender<MonitorEvent>,
}

impl AppState {
    fn new(event_tx: Sender<MonitorEvent>) -> Self {
        AppState {
            conn: unsafe { CGSMainConnectionID() },
            current_space: Cell::new(0),
            event_tx,
        }
    }

    fn update_active_space_number(&self) -> i32 {
        unsafe {
            let displays = CGSCopyManagedDisplaySpaces(self.conn);
            let active_display = CGSCopyActiveMenuBarDisplayIdentifier(self.conn);
            let displays_array: id = msg_send![class!(NSArray), arrayWithArray:displays];

            let mut active_space_id = -1;
            let mut all_spaces = Vec::new();

            let count: usize = msg_send![displays_array, count];
            for i in 0..count {
                let display: id = msg_send![displays_array, objectAtIndex:i];

                let current_space = make_nsstring("Current Space");
                let spaces = make_nsstring("Spaces");
                let disp_id = make_nsstring("Display Identifier");

                let current: id = msg_send![display, objectForKey:current_space];
                let spaces_arr: id = msg_send![display, objectForKey:spaces];
                let disp_identifier: id = msg_send![display, objectForKey:disp_id];

                if current.is_null() || spaces_arr.is_null() || disp_identifier.is_null() {
                    continue;
                }

                let disp_str: id = msg_send![disp_identifier, description];
                let main_str = make_nsstring("Main");
                let active_str = make_nsstring(&active_display.to_string());
                let is_main: BOOL = msg_send![disp_str, isEqualToString:main_str];
                let is_active: BOOL = msg_send![disp_str, isEqualToString:active_str];

                if is_main == YES || is_active == YES {
                    let space_id_key = make_nsstring("ManagedSpaceID");
                    active_space_id = msg_send![current, objectForKey:space_id_key];
                }

                let spaces_count: usize = msg_send![spaces_arr, count];
                for j in 0..spaces_count {
                    let space: id = msg_send![spaces_arr, objectAtIndex:j];
                    let tile_key = make_nsstring("TileLayoutManager");
                    let tile_layout: id = msg_send![space, objectForKey:tile_key];

                    if tile_layout.is_null() {
                        all_spaces.push(space);
                    }
                }
            }

            if active_space_id == -1 {
                println!("No active space found.");
                return -1;
            }

            for (index, space) in all_spaces.iter().enumerate() {
                let space_id_key = make_nsstring("ManagedSpaceID");
                let space_id: i32 = msg_send![*space, objectForKey:space_id_key];
                let space_number = index + 1;

                if space_id == active_space_id {
                    let prev_space = self.current_space.get();
                    self.current_space.set(space_number as i32);

                    if prev_space != space_number as i32 && prev_space != 0 {
                        let _ = self
                            .event_tx
                            .send(MonitorEvent::SpaceChange(space_number as i32));
                    }
                    return space_number as i32;
                }
            }
            -1
        }
    }
}

struct AppDelegate {
    state: AppState,
    _delegate: id,
}

impl AppDelegate {
    fn new(event_tx: Sender<MonitorEvent>) -> Self {
        let state = AppState::new(event_tx);

        unsafe {
            let mut decl =
                objc::declare::ClassDecl::new("RustAppDelegate", class!(NSObject)).unwrap();

            decl.add_ivar::<*mut c_void>("_rustState");

            extern "C" fn update_active_space_number(
                this: &Object,
                _sel: objc::runtime::Sel,
                _notification: id,
            ) {
                unsafe {
                    let state_ptr: *mut c_void = *this.get_ivar("_rustState");
                    let state = &*(state_ptr as *const AppState);
                    state.update_active_space_number();
                }
            }

            decl.add_method(
                sel!(updateActiveSpaceNumber:),
                update_active_space_number as extern "C" fn(&Object, _, _),
            );

            decl.register();

            let delegate_class = class!(RustAppDelegate);
            let delegate: id = msg_send![delegate_class, new];

            let app_delegate = AppDelegate {
                state,
                _delegate: delegate,
            };

            let state_ptr = &app_delegate.state as *const _ as *mut c_void;
            (*delegate).set_ivar("_rustState", state_ptr);

            app_delegate
        }
    }

    fn setup_application(&self) {
        unsafe {
            let app: id = msg_send![class!(NSApplication), sharedApplication];
            let _: () = msg_send![app, setActivationPolicy:
                NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory];
        }
    }

    fn setup_observers(&self) {
        unsafe {
            let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
            let notification_center: id = msg_send![workspace, notificationCenter];
            let active_space_name = make_nsstring("NSWorkspaceActiveSpaceDidChangeNotification");

            let _: () = msg_send![notification_center,
                addObserver:self._delegate
                selector:sel!(updateActiveSpaceNumber:)
                name:active_space_name
                object:workspace];
        }
    }

    fn start_listening(self) {
        self.setup_application();
        self.setup_observers();
        self.state.update_active_space_number();

        unsafe {
            let app: id = msg_send![class!(NSApplication), sharedApplication];
            let _: () = msg_send![app, setDelegate:self._delegate];
            let _: () = msg_send![app, run];
        }
    }
}
