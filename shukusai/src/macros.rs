macro_rules! assert_size_of {
	($($type:ty => $size:expr),*) => {
		$(
			assert_eq!(std::mem::size_of::<$type>(), $size);
		)*
	};
}
pub(crate) use assert_size_of;
