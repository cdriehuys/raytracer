//! An object identification mechanism that allows easy shape comparison by
//! comparing IDs instead of attributes.
//!
//! Inspired by garfieldnate's implementation:
//! https://github.com/garfieldnate/ray_tracer_challenge/blob/master/lib/src/object_id.rs

use std::sync::atomic::AtomicUsize;

/// A counter tracking the ID of the next object that gets created. Since we
/// only need uniqueness, a simple sequential counter is enough.
static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn next_object_id() -> usize {
    OBJECT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[derive(Copy, Debug, PartialEq)]
pub struct ObjectID {
    id: usize,
}

impl ObjectID {
    /// Get the object's ID.
    ///
    /// This is exposed as a read-only property to enforce the requirement that
    /// all new IDs be generated with the global counter.
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Clone for ObjectID {
    fn clone(&self) -> Self {
        // When we clone, we don't want to have the same ID, we want to generate
        // a new one. This lets us use the default Clone behavior in objects
        // that utilize ObjectID and maintain the correct behavior of clones
        // being separate objects.
        Self::default()
    }
}

impl Default for ObjectID {
    fn default() -> Self {
        Self {
            id: next_object_id(),
        }
    }
}
