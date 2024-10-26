// CurrentSpace-types.swift
import Cocoa

// Private CGS API declarations
@_silgen_name("CGSMainConnectionID")
public func CGSMainConnectionID() -> CGConnectionID

@_silgen_name("CGSCopyManagedDisplaySpaces") 
public func CGSCopyManagedDisplaySpaces(_ connection: CGConnectionID) -> CFArray

@_silgen_name("CGSCopyActiveMenuBarDisplayIdentifier")
public func CGSCopyActiveMenuBarDisplayIdentifier(_ connection: CGConnectionID) -> CFString

// Type aliases for CGS types
public typealias CGConnectionID = UInt32
