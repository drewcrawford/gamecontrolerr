# Drew's GameController.framework bindings for Rust

This library binds (some subset of) Apple GameController.framework to Rust.  As far as I know,
this is the only bindings of the framework in the Rust ecosystem.

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features
of gamecontrollerr

* Zero-cost abstractions.  Calling this library should perform identically to calling the framework from Swift/ObjC applications.
    * Most of the magic happens in [objr](https://github.com/drewcrawford/objr) or [foundationr](https://github.com/drewcrawford/foundationr)
      which provide cutting-edge high-performance primitives which are used here extensively.
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with platform APIs and are not documented separately.
* Free for noncommercial or "small commercial" use.

# Table of contents

The scope of this project is to bind "all of the framework", however, in practice, I mostly have time
to implement APIs I actually use in other projects.

The following APIs are at least partially supported:

* `GCKeyboard`, `GCKeyboardInput`
* `GCControllerButtonInput` - primarily `isPressed`
* `GCKeyCode` - all input codes.
