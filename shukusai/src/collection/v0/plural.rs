//---------------------------------------------------------------------------------------------------- Use
use crate::collection::v0::{Album, Artist, Song};
use crate::collection::{AlbumKey, ArtistKey, SongKey};
use bincode::{Decode, Encode};

//---------------------------------------------------------------------------------------------------- Plural newtypes around `Vec<T>`.
macro_rules! impl_plural {
    ($name:ident, $plural:ident, $key:ident) => {
        paste::paste! {
            #[derive(Clone,Debug,PartialEq,PartialOrd,Encode,Decode)]
            /// Type-safe wrapper around a [`Box`]'ed [`slice`].
            ///
            #[doc = "This struct's inner value is just `Box<[" $name "]>`"]
            ///
            /// This reimplements common [`slice`] functions/traits, notably [`std::ops::Index`]. This allows for type-safe indexing.
            ///
            /// For example, [`Albums`] is ONLY allowed to be indexed with an [`AlbumKey`]:
            /// ```rust,ignore
            /// let my_usize = 0;
            /// let key = AlbumKey::from(my_usize);
            ///
            /// // NOT type-safe, compile error!.
            /// collection.albums[my_usize];
            ///
            /// // Type-safe, compiles.
            /// collection.albums[key];
            ///```
            #[doc = "[`Collection`] itself can also be directly index with [`" $key "`]."]
            //-------------------------------------------------- Define plural `struct`.
            pub(crate) struct $plural(pub(crate) Box<[$name]>);

            //-------------------------------------------------- Implement `[]` indexing.
            impl std::ops::Index<$key> for $plural {
                type Output = $name;

                #[inline(always)]
                #[doc = "Index [`" $plural "`] with [`" $key "`]."]
                ///
                /// # Panics:
                /// The key must be a valid index.
                fn index(&self, key: $key) -> &Self::Output {
                    &self.0[key.inner()]
                }
            }
            impl std::ops::Index<&$key> for $plural {
                type Output = $name;

                #[inline(always)]
                #[doc = "Index [`" $plural "`] with [`" $key "`]."]
                ///
                /// # Panics:
                /// The key must be a valid index.
                fn index(&self, key: &$key) -> &Self::Output {
                    &self.0[key.inner()]
                }
            }

            impl $plural {
                //-------------------------------------------------- `pub(crate)` functions
                #[inline(always)]
                pub(crate) fn new() -> Self {
                    Self(Box::new([]))
                }
            }

            impl Into<crate::collection::$plural> for $plural {
                fn into(self) -> crate::collection::$plural {
                    let vec = Vec::from(self.0);

                    crate::collection::$plural(vec
                        .into_iter()
                        .enumerate()
                        .map(|(k, v)| {
                            let mut v: crate::collection::$name = v.into();
                            v.key = $key::from(k);
                            v
                        })
                        .collect()
                    )
                }
            }
        }
    };
}

impl_plural!(Artist, Artists, ArtistKey);
impl_plural!(Album, Albums, AlbumKey);
impl_plural!(Song, Songs, SongKey);
