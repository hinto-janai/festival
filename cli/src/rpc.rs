//---------------------------------------------------------------------------------------------------- Use
use crate::config::Config;
use rpc::Rpc;
use zeroize::Zeroize;
use crate::constants::FESTIVAL_CLI_USER_AGENT;

//---------------------------------------------------------------------------------------------------- Request
// `exit` is used to prevent destructors from running.
// We are exiting the program anyway so they don't need to run.
pub fn request(config: Config, debug: bool, dry_run: bool, rpc: Rpc) -> ! {
	if debug {
		eprintln!("=================================================> Config\n{}\n", serde_json::to_string_pretty(&config).unwrap());
	}

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
		($rpc:expr, $debug:expr, $expected_response:ty) => {{
			let rpc = $rpc.request(config.id);
			if $debug {
				eprintln!("=================================================> Request\n{}\n", serde_json::to_string_pretty(&rpc).unwrap());
			}

			if dry_run {
				eprintln!("=================================================> Aborting due to dry run");
				std::process::exit(0);
			}

			// Send request.
			let resp = match req.send_json(rpc) {
				Ok(s)  => s,
				Err(e) => crate::exit!("{e}"),
			};

			if $debug {
				eprintln!("=================================================> Response Info\n{{");
				for header in resp.headers_names() {
					match resp.header(&header) {
						Some(v) => eprintln!(r#"  "{header}": "{v}","#),
						None    => eprintln!(r#"  "{header}": "","#),
					}
				}
				eprintln!(r#"  "status": "{}","#, resp.status());
				eprintln!(r#"  "status-text": "{}","#, resp.status_text());
				eprintln!(r#"  "url": "{}""#, resp.get_url());
				eprintln!("}}\n");
			}

			// Parse response.
			let string = match resp.into_string() {
				Ok(s)  => s,
				Err(e) => crate::exit!("{e}"),
			};

			if debug {
				eprintln!("=================================================> Response");
			}

			// Check if response type is
			// correct, print, and exit.
			match serde_json::from_str::<json_rpc::Response<$expected_response>>(&string) {
				Ok(_) => {
					println!("{string}");
					std::process::exit(0);
				},

				Err(err) => {
					if cfg!(debug_assertions) || debug {
						eprintln!("=================================================> Error Response");
						eprintln!("{string}");
					}
					crate::exit!("{err}");
				},
			}
		}}
	}

	// Dispatch into proper method,
	use rpc::Rpc::*;
	match rpc {
		CollectionNew(x)          => req_resp!(x, debug, rpc::resp::CollectionNew),
		CollectionBrief(x)        => req_resp!(x, debug, rpc::resp::CollectionBrief),
		CollectionFull(x)         => req_resp!(x, debug, rpc::resp::CollectionFull),
		CollectionRelation(x)     => req_resp!(x, debug, rpc::resp::CollectionRelation),
		CollectionRelationFull(x) => req_resp!(x, debug, rpc::resp::CollectionRelationFull),
		CollectionPerf(x)         => req_resp!(x, debug, rpc::resp::CollectionPerf),
		CollectionResourceSize(x) => req_resp!(x, debug, rpc::resp::CollectionResourceSize),

		StateIp(x)     => req_resp!(x, debug, rpc::resp::StateIp),
		StateConfig(x) => req_resp!(x, debug, rpc::resp::StateConfig),
		StateDaemon(x) => req_resp!(x, debug, rpc::resp::StateDaemon),
		StateAudio(x)  => req_resp!(x, debug, rpc::resp::StateAudio),
		StateReset(x)  => req_resp!(x, debug, rpc::resp::StateReset),

		KeyArtist(x) => req_resp!(x, debug, rpc::resp::KeyArtist),
		KeyAlbum(x)  => req_resp!(x, debug, rpc::resp::KeyAlbum),
		KeySong(x)   => req_resp!(x, debug, rpc::resp::KeySong),

		MapArtist(x) => req_resp!(x, debug, rpc::resp::MapArtist),
		MapAlbum(x)  => req_resp!(x, debug, rpc::resp::MapAlbum),
		MapSong(x)   => req_resp!(x, debug, rpc::resp::MapSong),

		CurrentArtist(x) => req_resp!(x, debug, rpc::resp::CurrentArtist),
		CurrentAlbum(x)  => req_resp!(x, debug, rpc::resp::CurrentAlbum),
		CurrentSong(x)   => req_resp!(x, debug, rpc::resp::CurrentSong),

		RandArtist(x) => req_resp!(x, debug, rpc::resp::RandArtist),
		RandAlbum(x)  => req_resp!(x, debug, rpc::resp::RandAlbum),
		RandSong(x)   => req_resp!(x, debug, rpc::resp::RandSong),

		Search(x)       => req_resp!(x, debug, rpc::resp::Search),
		SearchArtist(x) => req_resp!(x, debug, rpc::resp::SearchArtist),
		SearchAlbum(x)  => req_resp!(x, debug, rpc::resp::SearchAlbum),
		SearchSong(x)   => req_resp!(x, debug, rpc::resp::SearchSong),

		Toggle(x)      => req_resp!(x, debug, rpc::resp::Status),
		Play(x)        => req_resp!(x, debug, rpc::resp::Status),
		Pause(x)       => req_resp!(x, debug, rpc::resp::Status),
		Next(x)        => req_resp!(x, debug, rpc::resp::Status),
		Stop(x)        => req_resp!(x, debug, rpc::resp::Status),
		Shuffle(x)     => req_resp!(x, debug, rpc::resp::Status),
		RepeatOff(x)   => req_resp!(x, debug, rpc::resp::Status),
		RepeatSong(x)  => req_resp!(x, debug, rpc::resp::Status),
		RepeatQueue(x) => req_resp!(x, debug, rpc::resp::Status),
		Previous(x)    => req_resp!(x, debug, rpc::resp::Status),
		Volume(x)      => req_resp!(x, debug, rpc::resp::Status),
		Clear(x)       => req_resp!(x, debug, rpc::resp::Status),
		Seek(x)        => req_resp!(x, debug, rpc::resp::Status),
		Skip(x)        => req_resp!(x, debug, rpc::resp::Status),
		Back(x)        => req_resp!(x, debug, rpc::resp::Status),

		QueueAddKeyArtist(x)  => req_resp!(x, debug, rpc::resp::Status),
		QueueAddKeyAlbum(x)   => req_resp!(x, debug, rpc::resp::Status),
		QueueAddKeySong(x)    => req_resp!(x, debug, rpc::resp::Status),
		QueueAddMapArtist(x)  => req_resp!(x, debug, rpc::resp::Status),
		QueueAddMapAlbum(x)   => req_resp!(x, debug, rpc::resp::Status),
		QueueAddMapSong(x)    => req_resp!(x, debug, rpc::resp::Status),
		QueueAddRandArtist(x) => req_resp!(x, debug, rpc::resp::QueueAddRandArtist),
		QueueAddRandAlbum(x)  => req_resp!(x, debug, rpc::resp::QueueAddRandAlbum),
		QueueAddRandSong(x)   => req_resp!(x, debug, rpc::resp::QueueAddRandSong),
		QueueAddPlaylist(x)   => req_resp!(x, debug, rpc::resp::Status),
		QueueSetIndex(x)      => req_resp!(x, debug, rpc::resp::QueueSetIndex),
		QueueRemoveRange(x)   => req_resp!(x, debug, rpc::resp::QueueRemoveRange),

		PlaylistNew(x)          => req_resp!(x, debug, rpc::resp::PlaylistNew),
		PlaylistRemove(x)       => req_resp!(x, debug, rpc::resp::PlaylistRemove),
		PlaylistClone(x)        => req_resp!(x, debug, rpc::resp::PlaylistClone),
		PlaylistRemoveEntry(x)  => req_resp!(x, debug, rpc::resp::PlaylistRemoveEntry),
		PlaylistAddKeyArtist(x) => req_resp!(x, debug, rpc::resp::PlaylistAddKeyArtist),
		PlaylistAddKeyAlbum(x)  => req_resp!(x, debug, rpc::resp::PlaylistAddKeyAlbum),
		PlaylistAddKeySong(x)   => req_resp!(x, debug, rpc::resp::PlaylistAddKeySong),
		PlaylistAddMapArtist(x) => req_resp!(x, debug, rpc::resp::PlaylistAddMapArtist),
		PlaylistAddMapAlbum(x)  => req_resp!(x, debug, rpc::resp::PlaylistAddMapAlbum),
		PlaylistAddMapSong(x)   => req_resp!(x, debug, rpc::resp::PlaylistAddMapSong),
		PlaylistNames(x)        => req_resp!(x, debug, rpc::resp::PlaylistNames),
		PlaylistCount(x)        => req_resp!(x, debug, rpc::resp::PlaylistCount),
		PlaylistSingle(x)       => req_resp!(x, debug, rpc::resp::PlaylistSingle),
		PlaylistAll(x)          => req_resp!(x, debug, rpc::resp::PlaylistAll),
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
