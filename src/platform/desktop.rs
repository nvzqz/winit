#![cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd"
))]

use event::Event;
use event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow};

/// Additional methods on `EventLoop` that are specific to desktop platforms.
pub trait EventLoopExt {
    /// A type provided by the user that can be passed through `Event::UserEvent`.
    type UserEvent;

    /// Initializes the `winit` event loop.
    ///
    /// Unlike `run`, this function accepts non-`'static` (i.e. non-`move`) closures and returns
    /// control flow to the caller when `control_flow` is set to `ControlFlow::Exit`.
    fn run_return<F>(&mut self, event_handler: F)
        where F: FnMut(Event<Self::UserEvent>, &EventLoopWindowTarget<Self::UserEvent>, &mut ControlFlow);
}

impl<T> EventLoopExt for EventLoop<T> {
    type UserEvent = T;

    fn run_return<F>(&mut self, event_handler: F)
        where F: FnMut(Event<T>, &EventLoopWindowTarget<T>, &mut ControlFlow)
    {
        self.event_loop.run_return(event_handler)
    }
}
