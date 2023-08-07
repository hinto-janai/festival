//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use log::{error,info,warn,debug,trace};
use disk::Bincode2;
use std::collections::{
	BTreeMap,
	VecDeque,
};
use crate::{
	collection::{
		SongKey,Collection,
	},
	constants::{
		FESTIVAL,
		HEADER,
		FRONTEND_SUB_DIR,
		STATE_SUB_DIR,
		PLAYLIST_VERSION,
	},
};

//---------------------------------------------------------------------------------------------------- __NAME__
disk::bincode2!(Playlists, disk::Dir::Data, FESTIVAL, formatcp!("{FRONTEND_SUB_DIR}/{STATE_SUB_DIR}"), "playlists", HEADER, PLAYLIST_VERSION);
#[derive(Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Encode,Decode)]
struct Playlists(BTreeMap<String, VecDeque<PlaylistEntry>>);

enum PlaylistEntry {
	Key(SongKey),
	Missing(StringId),
}

//fn prep() {
//	let vec = vec_deque
//		.into_par_iter()
//		.map(|key| {
//			let (artist, album, song) = collection.walk(key);
//
//			(artist.name.clone(), album.title.clone(), song.title.clone())
//		})
//		.collect();
//
//	// Vec<(Arc<str>, Arc<str>, Arc<str>)>
//	// --- collection reset
//
//	let vec_deque: VecDeque<PlaylistEntry> = vec
//		.into_par_iter()
//		.map(|artist, album, song| {
//			if let Some(key) = collection.song(&artist, &album, &song) {
//				PlaylistEntry::Key(key)
//			} else {
//				PlaylistEntry::Missing(StringId { artist, album, song })
//			}
//		})
//		.collect();
//}
//
//
//struct StringId {
//	artist: Arc<str>,
//	album: Arc<str>,
//	song: Arc<str>,
//}
//
//
//pub fn list() {
//	for entry in vec_deque {
//		match entry {
//			PlaylistEntry::Key(key)   => {
//				if shukusai::validate::song(key) {
//					// keep entry
//				} else {
//					// convert missing
//				}
//			},
//			PlaylistEntry::Missing(s) => /* ... */
//		}
//	}
//}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
