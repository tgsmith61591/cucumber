//! Tools for mapping [`Event`]s.
//!
//! [`Event`]: crate::Event
//!
//! [Gherkin]: https://cucumber.io/docs/gherkin/reference/

use crate::event;

/// A trait to be used by [`Runner`]s to map an event from `.run`
pub trait MapEvent<T: Sized>: Clone {
    /// A function that maps an outbound event from a runner
    /// into another event. Default behavior is an identity
    /// function.
    fn map(&self, e: event::Event<T>) -> event::Event<T>;
}

/// A noop mapper
#[derive(Clone, Copy, Default, Debug)]
pub struct Identity {}
impl<T: Sized> MapEvent<T> for Identity {
    fn map(&self, e: event::Event<T>) -> event::Event<T> {
        e
    }
}
