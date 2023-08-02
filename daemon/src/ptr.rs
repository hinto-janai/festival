//---------------------------------------------------------------------------------------------------- Use
use shukusai::collection::Collection;
use std::sync::{
	Arc,
	atomic::{
		Ordering,
		AtomicPtr,
	},
};

//---------------------------------------------------------------------------------------------------- Connections
/// Why this exists:
/// `hyper`'s `Http::new()` takes a closure that _may_
/// execute multiple times. AKA, a TCP connection that
/// is requesting multiple requests, AKA:
///
/// - open rest in browser
/// - browser keeps the connection open
/// - everytime it refreshes, the `service_fn` function is run
///
/// If we give a static `Arc<Collection>`, this is a problem since
/// that connection will forever use a stale `Collection` and won't
/// get a potentially new `Collection`.
///
/// Hooking up back-channels between `Router` <-> and all the `task`'s
/// is a pain, so instead this atomic pointer acts as a sort of
/// generator, AKA, `deref` this and receive the latest `Arc<Collection>`.
///
/// Now, `task`'s are handed this pointer, which they will deref per
/// HTTP request, getting the latest `Collection` each HTTP call.
pub struct CollectionPtr(pub(crate) AtomicPtr<Arc<Collection>>);

impl CollectionPtr {
	/// "Generate" the inner `Arc<Collection>`
	///
	/// `task`'s will use this for each HTTP request.
	pub fn arc(&self) -> Arc<Collection> {
		// SAFETY:
		// The _ONLY_ "entity" _setting_ this pointer
		// should be the `Router`, after receiving
		// a new `Collection` from the `collection_new` task.
		//
		// It must also _always_ be set to something, even `Collection::dummy()`,
		// meaning, we cannot overwrite the _actual_ `Collection` until
		// the `CollectionPtr` is pointing to the dummy.
		unsafe { Arc::clone(&*self.0.load(Ordering::Relaxed)) }
	}
}
