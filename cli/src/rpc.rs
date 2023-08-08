//---------------------------------------------------------------------------------------------------- Use
use crate::config::Config;
use rpc::Rpc;
use zeroize::Zeroize;
use crate::constants::FESTIVAL_CLI_USER_AGENT;

//---------------------------------------------------------------------------------------------------- Request
// `exit` is used to prevent destructors from running.
// We are exiting the program anyway so they don't need to run.
pub fn request(config: Config, rpc: Rpc) -> ! {
	// Create request.
	let mut req = ureq::post(&config.festivald)
		.set("User-Agent", FESTIVAL_CLI_USER_AGENT);

	// Add authorization.
	let req = match config.authorization {
		None    => req,
		Some(mut a) => {
			let req = req.set("Authorization", a.as_str());
			a.zeroize();
			req
		},
	};


	// Add timeout.
	let req = match config.timeout {
		None    => req,
		Some(t) => req.timeout(t),
	};

	macro_rules! req_resp {
		($rpc:expr, $expected_response:ty) => {{
			// Send request.
			let resp = match req.send_json($rpc.request(config.id)) {
				Ok(s)  => s,
				Err(e) => crate::exit!("{e}"),
			};

			// Parse response.
			let string = match resp.into_string() {
				Ok(s)  => s,
				Err(e) => crate::exit!("{e}"),
			};

			// Check if response type is
			// correct, print, and exit.
			match serde_json::from_str::<json_rpc::Response<$expected_response>>(&string) {
				Ok(_) => {
					println!("{string}");
					std::process::exit(0);
				},

				Err(err) => {
					#[cfg(debug_assertions)]
					eprintln!("{string}");
					crate::exit!("{err}");
				},
			}
		}}
	}

	// Dispatch into proper method,
	use rpc::Rpc::*;
	match rpc {
		CollectionNew(x)          => req_resp!(x, rpc::resp::CollectionNew),
		CollectionBrief(x)        => req_resp!(x, rpc::resp::CollectionBrief),
		CollectionFull(x)         => req_resp!(x, rpc::resp::CollectionFull),
		CollectionRelation(x)     => req_resp!(x, rpc::resp::CollectionRelation),
		CollectionRelationFull(x) => req_resp!(x, rpc::resp::CollectionRelationFull),
		CollectionPerf(x)         => req_resp!(x, rpc::resp::CollectionPerf),
		CollectionResourceSize(x) => req_resp!(x, rpc::resp::CollectionResourceSize),

		StateIp(x)     => req_resp!(x, rpc::resp::StateIp),
		StateConfig(x) => req_resp!(x, rpc::resp::StateConfig),
		StateDaemon(x) => req_resp!(x, rpc::resp::StateDaemon),
		StateAudio(x)  => req_resp!(x, rpc::resp::StateAudio),
		StateReset(x)  => req_resp!(x, rpc::resp::StateReset),

		KeyArtist(x) => req_resp!(x, rpc::resp::KeyArtist),
		KeyAlbum(x)  => req_resp!(x, rpc::resp::KeyAlbum),
		KeySong(x)   => req_resp!(x, rpc::resp::KeySong),

		MapArtist(x) => req_resp!(x, rpc::resp::MapArtist),
		MapAlbum(x)  => req_resp!(x, rpc::resp::MapAlbum),
		MapSong(x)   => req_resp!(x, rpc::resp::MapSong),

		CurrentArtist(x) => req_resp!(x, rpc::resp::CurrentArtist),
		CurrentAlbum(x)  => req_resp!(x, rpc::resp::CurrentAlbum),
		CurrentSong(x)   => req_resp!(x, rpc::resp::CurrentSong),

		RandArtist(x) => req_resp!(x, rpc::resp::RandArtist),
		RandAlbum(x)  => req_resp!(x, rpc::resp::RandAlbum),
		RandSong(x)   => req_resp!(x, rpc::resp::RandSong),

		Search(x)       => req_resp!(x, rpc::resp::Search),
		SearchArtist(x) => req_resp!(x, rpc::resp::SearchArtist),
		SearchAlbum(x)  => req_resp!(x, rpc::resp::SearchAlbum),
		SearchSong(x)   => req_resp!(x, rpc::resp::SearchSong),

		Toggle(x)      => req_resp!(x, rpc::resp::Status),
		Play(x)        => req_resp!(x, rpc::resp::Status),
		Pause(x)       => req_resp!(x, rpc::resp::Status),
		Next(x)        => req_resp!(x, rpc::resp::Status),
		Stop(x)        => req_resp!(x, rpc::resp::Status),
		Shuffle(x)     => req_resp!(x, rpc::resp::Status),
		RepeatOff(x)   => req_resp!(x, rpc::resp::Status),
		RepeatSong(x)  => req_resp!(x, rpc::resp::Status),
		RepeatQueue(x) => req_resp!(x, rpc::resp::Status),
		Previous(x)    => req_resp!(x, rpc::resp::Status),
		Volume(x)      => req_resp!(x, rpc::resp::Status),
		Clear(x)       => req_resp!(x, rpc::resp::Status),
		Seek(x)        => req_resp!(x, rpc::resp::Status),
		Skip(x)        => req_resp!(x, rpc::resp::Status),
		Back(x)        => req_resp!(x, rpc::resp::Status),

		QueueAddKeyArtist(x)  => req_resp!(x, rpc::resp::Status),
		QueueAddKeyAlbum(x)   => req_resp!(x, rpc::resp::Status),
		QueueAddKeySong(x)    => req_resp!(x, rpc::resp::Status),
		QueueAddMapArtist(x)  => req_resp!(x, rpc::resp::Status),
		QueueAddMapAlbum(x)   => req_resp!(x, rpc::resp::Status),
		QueueAddMapSong(x)    => req_resp!(x, rpc::resp::Status),
		QueueAddRandArtist(x) => req_resp!(x, rpc::resp::QueueAddRandArtist),
		QueueAddRandAlbum(x)  => req_resp!(x, rpc::resp::QueueAddRandAlbum),
		QueueAddRandSong(x)   => req_resp!(x, rpc::resp::QueueAddRandSong),
		QueueAddPlaylist(x)   => req_resp!(x, rpc::resp::Status),
		QueueSetIndex(x)      => req_resp!(x, rpc::resp::QueueSetIndex),
		QueueRemoveRange(x)   => req_resp!(x, rpc::resp::QueueRemoveRange),

		PlaylistNew(x)          => req_resp!(x, rpc::resp::PlaylistNew),
		PlaylistRemove(x)       => req_resp!(x, rpc::resp::PlaylistRemove),
		PlaylistClone(x)        => req_resp!(x, rpc::resp::PlaylistClone),
		PlaylistRemoveEntry(x)  => req_resp!(x, rpc::resp::PlaylistRemoveEntry),
		PlaylistAddKeyArtist(x) => req_resp!(x, rpc::resp::PlaylistAddKeyArtist),
		PlaylistAddKeyAlbum(x)  => req_resp!(x, rpc::resp::PlaylistAddKeyAlbum),
		PlaylistAddKeySong(x)   => req_resp!(x, rpc::resp::PlaylistAddKeySong),
		PlaylistAddMapArtist(x) => req_resp!(x, rpc::resp::PlaylistAddMapArtist),
		PlaylistAddMapAlbum(x)  => req_resp!(x, rpc::resp::PlaylistAddMapAlbum),
		PlaylistAddMapSong(x)   => req_resp!(x, rpc::resp::PlaylistAddMapSong),
		PlaylistNames(x)        => req_resp!(x, rpc::resp::PlaylistNames),
		PlaylistCount(x)        => req_resp!(x, rpc::resp::PlaylistCount),
		PlaylistSingle(x)       => req_resp!(x, rpc::resp::PlaylistSingle),
		PlaylistAll(x)          => req_resp!(x, rpc::resp::PlaylistAll),
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
