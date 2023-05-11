/* Bincode decoding code, for use with
 * translating indicies into the `*Ptr` types.
 *
 * As long as this comment is here, this is not
 * actually in use. If the day comes where the
 * `*Ptr` types are actually used, this comment
 * will be removed and this file will be added
 * to the `mod.rs` file.
 */

//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{
	Collection,
	Artists,Albums,Songs,
	ArtistPtr,AlbumPtr,SongPtr,
	ArtistKey,AlbumKey,SongKey,
};
use bincode::{
	de::{Decode,Decoder,BorrowDecode,BorrowDecoder},
	enc::{Encode,Encoder},
	error::{DecodeError,EncodeError},
};

//---------------------------------------------------------------------------------------------------- Decode
// Custom `bincode::Decode` for `Collection`.
//
// INVARIANT:
// The `*Ptr` types within `Collection` cannot reliably be saved to disk,
// so must use the `*Key` types to find our way, acquire a pointer, and
// overwrite the default `nullptr` that was assigned to it.
//
// This invariant must be upheld or else `Collection`'s pointers will all
// be `nullptr`'s, although, as long as this process goes well, all the
// pointers within `Collection` are valid for the lifetime of the `Collection` itself.
impl Decode for Collection {
	#[inline(always)]
	fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
		let artists = Decode::decode(decoder)?;
		let albums  = Decode::decode(decoder)?;
		let songs   = Decode::decode(decoder)?;
		let map     = Decode::decode(decoder)?;

		let sort_artist_lexi        = ArtistPtr::decode(&artists, Decode::decode(decoder)?);
		let sort_artist_album_count = ArtistPtr::decode(&artists, Decode::decode(decoder)?);
		let sort_artist_song_count  = ArtistPtr::decode(&artists, Decode::decode(decoder)?);

		let sort_album_release_artist_lexi = AlbumPtr::decode(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_artist_lexi    = AlbumPtr::decode(&albums, Decode::decode(decoder)?);
		let sort_album_lexi                = AlbumPtr::decode(&albums, Decode::decode(decoder)?);
		let sort_album_release             = AlbumPtr::decode(&albums, Decode::decode(decoder)?);
		let sort_album_runtime             = AlbumPtr::decode(&albums, Decode::decode(decoder)?);

		let sort_song_album_release_artist_lexi = SongPtr::decode(&songs, Decode::decode(decoder)?);
		let sort_song_album_lexi_artist_lexi    = SongPtr::decode(&songs, Decode::decode(decoder)?);
		let sort_song_lexi                      = SongPtr::decode(&songs, Decode::decode(decoder)?);
		let sort_song_release                   = SongPtr::decode(&songs, Decode::decode(decoder)?);
		let sort_song_runtime                   = SongPtr::decode(&songs, Decode::decode(decoder)?);

		Ok(Self {
			map,
			artists,
			albums,
			songs,

			sort_artist_lexi,
			sort_artist_album_count,
			sort_artist_song_count,

			sort_album_release_artist_lexi,
			sort_album_lexi_artist_lexi,
			sort_album_lexi,
			sort_album_release,
			sort_album_runtime,

			sort_song_album_release_artist_lexi,
			sort_song_album_lexi_artist_lexi,
			sort_song_lexi,
			sort_song_release,
			sort_song_runtime,

			empty: Decode::decode(decoder)?,
			timestamp: Decode::decode(decoder)?,
			count_artist: Decode::decode(decoder)?,
			count_album: Decode::decode(decoder)?,
			count_song: Decode::decode(decoder)?,
			count_art: Decode::decode(decoder)?,
		})
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
