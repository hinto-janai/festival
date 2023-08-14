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
		eprintln!("\n=================================================> Config\n{}\n", serde_json::to_string_pretty(&config).unwrap());
	}

	// Create agent
	let agent = ureq::AgentBuilder::new()
		.user_agent(FESTIVAL_CLI_USER_AGENT);

	// Add timeout.
	let agent = match config.timeout {
		None    => agent,
		Some(t) => agent.timeout(t),
	};

	// Add proxy.
	let agent = match config.proxy {
		None    => agent,
		Some(p) => agent.proxy(p.proxy),
	};

	let req = agent.build().post(&config.festivald);

	// Add authorization.
	let req = match config.authorization {
		None    => req,
		Some(a) => req.set("authorization", a.as_str()),
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
		CollectionBriefArtists(x) => req_resp!(x, debug, rpc::resp::CollectionBriefArtists),
		CollectionBriefAlbums(x)  => req_resp!(x, debug, rpc::resp::CollectionBriefAlbums),
		CollectionBriefSongs(x)   => req_resp!(x, debug, rpc::resp::CollectionBriefSongs),
		CollectionFullArtists(x)  => req_resp!(x, debug, rpc::resp::CollectionFullArtists),
		CollectionFullAlbums(x)   => req_resp!(x, debug, rpc::resp::CollectionFullAlbums),
		CollectionFullSongs(x)    => req_resp!(x, debug, rpc::resp::CollectionFullSongs),
		CollectionEntries(x)      => req_resp!(x, debug, rpc::resp::CollectionEntries),
		CollectionPerf(x)         => req_resp!(x, debug, rpc::resp::CollectionPerf),
		CollectionHealth(x)       => req_resp!(x, debug, rpc::resp::CollectionHealth),
		CollectionResourceSize(x) => req_resp!(x, debug, rpc::resp::CollectionResourceSize),

		DiskSave(x)        => req_resp!(x, debug, rpc::resp::Status),
		DiskRemoveCache(x) => req_resp!(x, debug, rpc::resp::DiskRemoveCache),

		StateAudio(x)      => req_resp!(x, debug, rpc::resp::StateAudio),
		StateConfig(x)     => req_resp!(x, debug, rpc::resp::StateConfig),
		StateDaemon(x)     => req_resp!(x, debug, rpc::resp::StateDaemon),
		StateIp(x)         => req_resp!(x, debug, rpc::resp::StateIp),
		StateQueue(x)      => req_resp!(x, debug, rpc::resp::StateQueue),
		StateQueueEntry(x) => req_resp!(x, debug, rpc::resp::StateQueueEntry),
		StateReset(x)      => req_resp!(x, debug, rpc::resp::StateReset),
		StateVolume(x)     => req_resp!(x, debug, rpc::resp::StateVolume),

		KeyArtist(x) => req_resp!(x, debug, rpc::resp::KeyArtist),
		KeyAlbum(x)  => req_resp!(x, debug, rpc::resp::KeyAlbum),
		KeySong(x)   => req_resp!(x, debug, rpc::resp::KeySong),
		KeyEntry(x)  => req_resp!(x, debug, rpc::resp::KeyEntry),

		MapArtist(x) => req_resp!(x, debug, rpc::resp::MapArtist),
		MapAlbum(x)  => req_resp!(x, debug, rpc::resp::MapAlbum),
		MapSong(x)   => req_resp!(x, debug, rpc::resp::MapSong),
		MapEntry(x)  => req_resp!(x, debug, rpc::resp::MapEntry),

		CurrentArtist(x) => req_resp!(x, debug, rpc::resp::CurrentArtist),
		CurrentAlbum(x)  => req_resp!(x, debug, rpc::resp::CurrentAlbum),
		CurrentSong(x)   => req_resp!(x, debug, rpc::resp::CurrentSong),
		CurrentEntry(x)  => req_resp!(x, debug, rpc::resp::CurrentEntry),

		RandArtist(x) => req_resp!(x, debug, rpc::resp::RandArtist),
		RandAlbum(x)  => req_resp!(x, debug, rpc::resp::RandAlbum),
		RandSong(x)   => req_resp!(x, debug, rpc::resp::RandSong),
		RandEntry(x)  => req_resp!(x, debug, rpc::resp::RandEntry),

		Search(x)       => req_resp!(x, debug, rpc::resp::Search),
		SearchArtist(x) => req_resp!(x, debug, rpc::resp::SearchArtist),
		SearchAlbum(x)  => req_resp!(x, debug, rpc::resp::SearchAlbum),
		SearchSong(x)   => req_resp!(x, debug, rpc::resp::SearchSong),
		SearchEntry(x)  => req_resp!(x, debug, rpc::resp::SearchEntry),

		Toggle(x)      => req_resp!(x, debug, rpc::resp::Status),
		Play(x)        => req_resp!(x, debug, rpc::resp::Status),
		Pause(x)       => req_resp!(x, debug, rpc::resp::Status),
		Next(x)        => req_resp!(x, debug, rpc::resp::Status),
		Stop(x)        => req_resp!(x, debug, rpc::resp::Status),
		Previous(x)    => req_resp!(x, debug, rpc::resp::Status),
		Clear(x)       => req_resp!(x, debug, rpc::resp::Status),
		Seek(x)        => req_resp!(x, debug, rpc::resp::Status),
		Skip(x)        => req_resp!(x, debug, rpc::resp::Status),
		Back(x)        => req_resp!(x, debug, rpc::resp::Status),
		Shuffle(x)     => req_resp!(x, debug, rpc::resp::Status),
		RepeatOff(x)   => req_resp!(x, debug, rpc::resp::Status),
		RepeatSong(x)  => req_resp!(x, debug, rpc::resp::Status),
		RepeatQueue(x) => req_resp!(x, debug, rpc::resp::Status),
		Volume(x)      => req_resp!(x, debug, rpc::resp::Volume),
		VolumeUp(x)    => req_resp!(x, debug, rpc::resp::VolumeUp),
		VolumeDown(x)  => req_resp!(x, debug, rpc::resp::VolumeDown),

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
		PlaylistSingle(x)       => req_resp!(x, debug, rpc::resp::PlaylistSingle),
		PlaylistBrief(x)        => req_resp!(x, debug, rpc::resp::PlaylistBrief),
		PlaylistFull(x)         => req_resp!(x, debug, rpc::resp::PlaylistFull),
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//	#[test]
//		fn __TEST__() {
//	}
//}
