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
	fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
		let empty        = Decode::decode(decoder)?;
		let timestamp    = Decode::decode(decoder)?;
		let count_artist = Decode::decode(decoder)?;
		let count_album  = Decode::decode(decoder)?;
		let count_song   = Decode::decode(decoder)?;
		let count_art    = Decode::decode(decoder)?;

		let map     = Decode::decode(decoder)?;
		let artists = Decode::decode(decoder)?;
		let albums  = Decode::decode(decoder)?;
		let songs   = Decode::decode(decoder)?;

		let sort_artist_lexi            = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_lexi_rev        = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_album_count     = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_album_count_rev = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_song_count      = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_song_count_rev  = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_runtime         = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_runtime_rev     = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_name            = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);
		let sort_artist_name_rev        = ArtistPtr::from_key(&artists, Decode::decode(decoder)?);

		let sort_album_release_artist_lexi         = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_release_artist_lexi_rev     = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_release_rev_artist_lexi     = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_release_rev_artist_lexi_rev = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_artist_lexi            = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_artist_lexi_rev        = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_rev_artist_lexi        = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_rev_artist_lexi_rev    = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi                        = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_lexi_rev                    = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_release                     = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_release_rev                 = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_runtime                     = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_runtime_rev                 = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_title                       = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);
		let sort_album_title_rev                   = AlbumPtr::from_key(&albums, Decode::decode(decoder)?);

		let sort_song_album_release_artist_lexi         = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_release_artist_lexi_rev     = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_release_rev_artist_lexi     = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_release_rev_artist_lexi_rev = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_lexi_artist_lexi            = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_lexi_artist_lexi_rev        = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_lexi_rev_artist_lexi        = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_album_lexi_rev_artist_lexi_rev    = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_lexi                              = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_lexi_rev                          = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_release                           = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_release_rev                       = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_runtime                           = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_runtime_rev                       = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_title                             = SongPtr::from_key(&songs, Decode::decode(decoder)?);
		let sort_song_title_rev                         = SongPtr::from_key(&songs, Decode::decode(decoder)?);

		Ok(Self {
			empty,
			timestamp,
			count_artist,
			count_album,
			count_song,
			count_art,

			map,
			artists,
			albums,
			songs,

			sort_artist_lexi,
			sort_artist_lexi_rev,
			sort_artist_album_count,
			sort_artist_album_count_rev,
			sort_artist_song_count,
			sort_artist_song_count_rev,
			sort_artist_runtime,
			sort_artist_runtime_rev,
			sort_artist_name,
			sort_artist_name_rev,

			sort_album_release_artist_lexi,
			sort_album_release_artist_lexi_rev,
			sort_album_release_rev_artist_lexi,
			sort_album_release_rev_artist_lexi_rev,
			sort_album_lexi_artist_lexi,
			sort_album_lexi_artist_lexi_rev,
			sort_album_lexi_rev_artist_lexi,
			sort_album_lexi_rev_artist_lexi_rev,
			sort_album_lexi,
			sort_album_lexi_rev,
			sort_album_release,
			sort_album_release_rev,
			sort_album_runtime,
			sort_album_runtime_rev,
			sort_album_title,
			sort_album_title_rev,

			sort_song_album_release_artist_lexi,
			sort_song_album_release_artist_lexi_rev,
			sort_song_album_release_rev_artist_lexi,
			sort_song_album_release_rev_artist_lexi_rev,
			sort_song_album_lexi_artist_lexi,
			sort_song_album_lexi_artist_lexi_rev,
			sort_song_album_lexi_rev_artist_lexi,
			sort_song_album_lexi_rev_artist_lexi_rev,
			sort_song_lexi,
			sort_song_lexi_rev,
			sort_song_release,
			sort_song_release_rev,
			sort_song_runtime,
			sort_song_runtime_rev,
			sort_song_title,
			sort_song_title_rev,
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
