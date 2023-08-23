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

		DaemonConfig(x)      => req_resp!(x, debug, rpc::resp::DaemonConfig),
		DaemonMethods(x)     => req_resp!(x, debug, rpc::resp::DaemonMethods),
		DaemonNoAuthRpc(x)   => req_resp!(x, debug, rpc::resp::DaemonNoAuthRpc),
		DaemonNoAuthRest(x)  => req_resp!(x, debug, rpc::resp::DaemonNoAuthRest),
		DaemonRemoveCache(x) => req_resp!(x, debug, rpc::resp::DaemonRemoveCache),
		DaemonSave(x)        => req_resp!(x, debug, rpc::resp::Status),
		DaemonSeenIps(x)     => req_resp!(x, debug, rpc::resp::DaemonSeenIps),
		DaemonShutdown(x)    => req_resp!(x, debug, rpc::resp::DaemonShutdown),
		DaemonState(x)       => req_resp!(x, debug, rpc::resp::DaemonState),

		StateAudio(x)      => req_resp!(x, debug, rpc::resp::StateAudio),
		StateQueueKey(x)   => req_resp!(x, debug, rpc::resp::StateQueueKey),
		StateQueueSong(x)  => req_resp!(x, debug, rpc::resp::StateQueueSong),
		StateQueueEntry(x) => req_resp!(x, debug, rpc::resp::StateQueueEntry),
		StatePlaying(x)    => req_resp!(x, debug, rpc::resp::StatePlaying),
		StateRepeat(x)     => req_resp!(x, debug, rpc::resp::StateRepeat),
		StateRuntime(x)    => req_resp!(x, debug, rpc::resp::StateRuntime),
		StateVolume(x)     => req_resp!(x, debug, rpc::resp::StateVolume),

		KeyArtist(x)        => req_resp!(x, debug, rpc::resp::KeyArtist),
		KeyAlbum(x)         => req_resp!(x, debug, rpc::resp::KeyAlbum),
		KeySong(x)          => req_resp!(x, debug, rpc::resp::KeySong),
		KeyEntry(x)         => req_resp!(x, debug, rpc::resp::KeyEntry),
		KeyArtistAlbums(x)  => req_resp!(x, debug, rpc::resp::KeyArtistAlbums),
		KeyArtistSongs(x)   => req_resp!(x, debug, rpc::resp::KeyArtistSongs),
		KeyArtistEntries(x) => req_resp!(x, debug, rpc::resp::KeyArtistEntries),
		KeyAlbumArtist(x)   => req_resp!(x, debug, rpc::resp::KeyAlbumArtist),
		KeyAlbumSongs(x)    => req_resp!(x, debug, rpc::resp::KeyAlbumSongs),
		KeyAlbumEntries(x)  => req_resp!(x, debug, rpc::resp::KeyAlbumEntries),
		KeySongArtist(x)    => req_resp!(x, debug, rpc::resp::KeySongArtist),
		KeySongAlbum(x)     => req_resp!(x, debug, rpc::resp::KeySongAlbum),
		KeyOtherAlbums(x)   => req_resp!(x, debug, rpc::resp::KeyOtherAlbums),
		KeyOtherSongs(x)    => req_resp!(x, debug, rpc::resp::KeyOtherSongs),
		KeyOtherEntries(x)  => req_resp!(x, debug, rpc::resp::KeyOtherEntries),

		MapArtist(x)        => req_resp!(x, debug, rpc::resp::MapArtist),
		MapAlbum(x)         => req_resp!(x, debug, rpc::resp::MapAlbum),
		MapSong(x)          => req_resp!(x, debug, rpc::resp::MapSong),
		MapEntry(x)         => req_resp!(x, debug, rpc::resp::MapEntry),
		MapArtistAlbums(x)  => req_resp!(x, debug, rpc::resp::MapArtistAlbums),
		MapArtistSongs(x)   => req_resp!(x, debug, rpc::resp::MapArtistSongs),
		MapArtistEntries(x) => req_resp!(x, debug, rpc::resp::MapArtistEntries),
		MapAlbumSongs(x)    => req_resp!(x, debug, rpc::resp::MapAlbumSongs),
		MapAlbumEntries(x)  => req_resp!(x, debug, rpc::resp::MapAlbumEntries),

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
		Clear(x)       => req_resp!(x, debug, rpc::resp::Clear),
		Stop(x)        => req_resp!(x, debug, rpc::resp::Stop),
		Next(x)        => req_resp!(x, debug, rpc::resp::Status),
		Previous(x)    => req_resp!(x, debug, rpc::resp::Status),
		Skip(x)        => req_resp!(x, debug, rpc::resp::Status),
		Back(x)        => req_resp!(x, debug, rpc::resp::Status),
		Seek(x)        => req_resp!(x, debug, rpc::resp::Status),
		Shuffle(x)     => req_resp!(x, debug, rpc::resp::Status),
		Repeat(x)      => req_resp!(x, debug, rpc::resp::Repeat),
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
		QueueAddRandEntry(x)  => req_resp!(x, debug, rpc::resp::QueueAddRandEntry),
		QueueAddPlaylist(x)   => req_resp!(x, debug, rpc::resp::Status),
		QueueSetIndex(x)      => req_resp!(x, debug, rpc::resp::QueueSetIndex),
		QueueRemoveRange(x)   => req_resp!(x, debug, rpc::resp::QueueRemoveRange),

		PlaylistNew(x)          => req_resp!(x, debug, rpc::resp::PlaylistNew),
		PlaylistRemove(x)       => req_resp!(x, debug, rpc::resp::PlaylistRemove),
		PlaylistClone(x)        => req_resp!(x, debug, rpc::resp::PlaylistClone),
		PlaylistGetIndex(x)     => req_resp!(x, debug, rpc::resp::PlaylistGetIndex),
		PlaylistRemoveIndex(x)  => req_resp!(x, debug, rpc::resp::PlaylistRemoveIndex),
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
#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;
	use shukusai::collection::Collection;

	macro_rules! test_method {
		($($method:expr => $expected_resp_type:ty, $params:expr, $expected_json_output:expr),*) => {{ $(
			println!("\n\n\n\n=================================================> Testing Method {:#?}\n", $method);

			let input_json = ureq::json!({
				"jsonrpc": "2.0",
				"id":0,
				"method": $method,
				"params": $params,
			});

			println!("=================================================> Input JSON\n{}\n", input_json);

			// Send request.
			let resp = match ureq::post("http://localhost:18425").send_json(input_json) {
				Ok(s)  => s,
				Err(e) => panic!("=================================================> Method [{:#?}] Post Error: {e}", $method),
			};

			println!("=================================================> Response Info\n{{");
			for header in resp.headers_names() {
				match resp.header(&header) {
					Some(v) => println!(r#"  "{header}": "{v}","#),
					None    => println!(r#"  "{header}": "","#),
				}
			}
			println!(r#"  "status": "{}","#, resp.status());
			println!(r#"  "status-text": "{}","#, resp.status_text());
			println!(r#"  "url": "{}""#, resp.get_url());
			println!("}}\n");

			// Parse response.
			let string = match resp.into_string() {
				Ok(s)  => s,
				Err(e) => panic!("=================================================> Method [{:#?}] String Parse Error: {e}\n", $method),
			};


			// Check if response type is correct.
			let resp = match serde_json::from_str::<json_rpc::Response<$expected_resp_type>>(&string) {
				Ok(resp) => { println!("=================================================> Response OK\n{string}\n"); resp },
				Err(err) => {
					println!("=================================================> Method [{:#?}] Resp Serde Error: {err}", $method);
					println!("{string}");
					panic!();
				},
			};

			// Assert output is `result` not `error`
			println!("================================================= Assert Result for [{:#?}]", $method);
			assert!(resp.payload.is_ok());

			// Assert it is the same as expected output.
			if !$expected_json_output.is_empty() {
				println!("================================================= Assert Output for [{:#?}]", $method);
				assert_eq!(string, $expected_json_output);
			}
		)* }}
	}

	#[test]
	#[ignore]
	// This launches `festivald`, and tests every
	// single JSON-RPC call for valid input/output
	// on the binary level with `festival-cli`.
	fn rpc() {
		// Define where `festivald` is.
		// It should be built in `--release` mode first.
		#[cfg(unix)]
		let festivald = PathBuf::from("../target/release/festivald");
		#[cfg(windows)]
		let festivald = PathBuf::from("../target/release/festivald.exe");

		assert!(festivald.exists());

		// Reset `festivald`'s config,
		std::process::Command::new(&festivald)
			.arg("--delete")
			.stdout(std::process::Stdio::null())
			.output()
			.unwrap();

		// Load the fake `Collection`.
		use disk::Bincode2;
		Collection::mkdir().unwrap();
		std::fs::copy("../assets/shukusai/state/collection3_real.bin", Collection::absolute_path().unwrap()).unwrap();

		// Spawn `festivald`.
		std::process::Command::new(&festivald)
			.args([
				"--ip",
				"127.0.0.1",
				"--port",
				"18425",
				"--disable-restore-audio-state",
				"--disable-watch",
				"--disable-media-controls",
				"--disable-rest",
				"--disable-docs",
				"--log-level",
				"off",
				"--exclusive-ip",
				"127.0.0.1",
			])
			.stdout(std::process::Stdio::null())
			.spawn()
			.unwrap();

		// Wait until `festivald` is online.
		let mut seen_ips = 0;
		loop {
			match ureq::post("http://localhost:18425").send_json(ureq::json!({"jsonrpc":"2.0","id":0,"method":"daemon_state"})) {
				Ok(_)  => break,
				Err(e) => {
					if seen_ips > 100 {
						panic!("seen_ips > 100, can't connect to festivald");
					}

					seen_ips += 1;
					std::thread::sleep(std::time::Duration::from_millis(100));
					println!("[{seen_ips}] Waiting on `festivald`...");
				},
			};
		}

		// Test every RPC call to check if:
		//   1. Method enun <-> string works
		//   2. Params work
		//   3. Response string can be deserialized as the expected type
		//   4. Response is a `JSON-RPC` `result`, not `error`
		//   5. (Optional) The response string is the exact same as the given string
		use rpc::Method::*;
		test_method! {
			// Can't test this.
			// We need the fake `Collection` loaded.
//			CollectionNew => rpc::resp::CollectionNew,           // rpc method => expected output type
//			ureq::json!({"paths":["/path/that/doesnt/exist/"]}), // parameters
//			"",                                                  // expected output json if non-empty

			CollectionBrief => rpc::resp::CollectionBrief,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1688690421,
    "count_artist": 3,
    "count_album": 4,
    "count_song": 7,
    "count_art": 4
  },
  "id": 0
}"#,

			CollectionFull => rpc::resp::CollectionFull,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "empty": false,
    "timestamp": 1688690421,
    "count_artist": 3,
    "count_album": 4,
    "count_song": 7,
    "count_art": 4,
    "artists": [
      {
        "name": "artist_1",
        "key": 0,
        "runtime": 4,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          2,
          3
        ]
      },
      {
        "name": "artist_2",
        "key": 1,
        "runtime": 2,
        "albums": [
          2
        ],
        "songs": [
          4,
          5
        ]
      },
      {
        "name": "artist_3",
        "key": 2,
        "runtime": 1,
        "albums": [
          3
        ],
        "songs": [
          6
        ]
      }
    ],
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_3",
        "key": 2,
        "artist": 1,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          4,
          5
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_4",
        "key": 3,
        "artist": 2,
        "release": "2018-04-25",
        "runtime": 1,
        "song_count": 1,
        "songs": [
          6
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ],
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "m4a",
        "key": 4,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "song_6",
        "key": 5,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 6,
        "album": 3,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ],
    "sort_artist_lexi": [
      0,
      1,
      2
    ],
    "sort_artist_lexi_rev": [
      2,
      1,
      0
    ],
    "sort_artist_album_count": [
      1,
      2,
      0
    ],
    "sort_artist_album_count_rev": [
      0,
      2,
      1
    ],
    "sort_artist_song_count": [
      2,
      1,
      0
    ],
    "sort_artist_song_count_rev": [
      0,
      1,
      2
    ],
    "sort_artist_runtime": [
      2,
      1,
      0
    ],
    "sort_artist_runtime_rev": [
      0,
      1,
      2
    ],
    "sort_artist_name": [
      0,
      1,
      2
    ],
    "sort_artist_name_rev": [
      2,
      1,
      0
    ],
    "sort_album_release_artist_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_release_artist_lexi_rev": [
      3,
      2,
      0,
      1
    ],
    "sort_album_release_rev_artist_lexi": [
      1,
      0,
      2,
      3
    ],
    "sort_album_release_rev_artist_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_lexi_artist_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_lexi_artist_lexi_rev": [
      3,
      2,
      0,
      1
    ],
    "sort_album_lexi_rev_artist_lexi": [
      1,
      0,
      2,
      3
    ],
    "sort_album_lexi_rev_artist_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_lexi": [
      0,
      1,
      2,
      3
    ],
    "sort_album_lexi_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_release": [
      0,
      1,
      2,
      3
    ],
    "sort_album_release_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_album_runtime": [
      3,
      0,
      1,
      2
    ],
    "sort_album_runtime_rev": [
      2,
      1,
      0,
      3
    ],
    "sort_album_title": [
      0,
      1,
      2,
      3
    ],
    "sort_album_title_rev": [
      3,
      2,
      1,
      0
    ],
    "sort_song_album_release_artist_lexi": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_album_release_artist_lexi_rev": [
      6,
      4,
      5,
      0,
      1,
      2,
      3
    ],
    "sort_song_album_release_rev_artist_lexi": [
      2,
      3,
      0,
      1,
      4,
      5,
      6
    ],
    "sort_song_album_release_rev_artist_lexi_rev": [
      6,
      4,
      5,
      2,
      3,
      0,
      1
    ],
    "sort_song_album_lexi_artist_lexi": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_album_lexi_artist_lexi_rev": [
      6,
      4,
      5,
      0,
      1,
      2,
      3
    ],
    "sort_song_album_lexi_rev_artist_lexi": [
      2,
      3,
      0,
      1,
      4,
      5,
      6
    ],
    "sort_song_album_lexi_rev_artist_lexi_rev": [
      6,
      4,
      5,
      2,
      3,
      0,
      1
    ],
    "sort_song_lexi": [
      3,
      4,
      0,
      1,
      2,
      6,
      5
    ],
    "sort_song_lexi_rev": [
      5,
      6,
      2,
      1,
      0,
      4,
      3
    ],
    "sort_song_release": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_release_rev": [
      6,
      5,
      4,
      3,
      2,
      1,
      0
    ],
    "sort_song_runtime": [
      0,
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "sort_song_runtime_rev": [
      6,
      5,
      4,
      3,
      2,
      1,
      0
    ],
    "sort_song_title": [
      0,
      1,
      2,
      4,
      6,
      3,
      5
    ],
    "sort_song_title_rev": [
      5,
      3,
      6,
      4,
      2,
      1,
      0
    ]
  },
  "id": 0
}"#,

			CollectionBriefArtists => rpc::resp::CollectionBriefArtists,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "artists": [
      "artist_1",
      "artist_2",
      "artist_3"
    ]
  },
  "id": 0
}"#,

			CollectionBriefAlbums => rpc::resp::CollectionBriefAlbums,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 4,
    "albums": [
      "album_1",
      "album_2",
      "album_3",
      "album_4"
    ]
  },
  "id": 0
}"#,

			CollectionBriefSongs => rpc::resp::CollectionBriefSongs,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 7,
    "songs": [
      "flac",
      "m4a",
      "mp3",
      "mp3",
      "mp3",
      "mp3",
      "song_6"
    ]
  },
  "id": 0
}"#,

			CollectionFullArtists => rpc::resp::CollectionFullArtists,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 3,
    "artists": [
      {
        "name": "artist_1",
        "key": 0,
        "runtime": 4,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          2,
          3
        ]
      },
      {
        "name": "artist_2",
        "key": 1,
        "runtime": 2,
        "albums": [
          2
        ],
        "songs": [
          4,
          5
        ]
      },
      {
        "name": "artist_3",
        "key": 2,
        "runtime": 1,
        "albums": [
          3
        ],
        "songs": [
          6
        ]
      }
    ]
  },
  "id": 0
}"#,

			CollectionFullAlbums => rpc::resp::CollectionFullAlbums,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 4,
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_3",
        "key": 2,
        "artist": 1,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          4,
          5
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_4",
        "key": 3,
        "artist": 2,
        "release": "2018-04-25",
        "runtime": 1,
        "song_count": 1,
        "songs": [
          6
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ]
  },
  "id": 0
}"#,

			CollectionFullSongs => rpc::resp::CollectionFullSongs,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 7,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "m4a",
        "key": 4,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "song_6",
        "key": 5,
        "album": 2,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 6,
        "album": 3,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			CollectionEntries => rpc::resp::CollectionEntries,
			"",
			"", // Empty, we don't know the full path on the system we're running this on.

			// Skipped.
//			CollectionPerf => rpc::resp::CollectionPerf,
//			"",
//			"",

			CollectionHealth => rpc::resp::CollectionHealth,
			"",
			"", // Empty, we don't know the full path on the system we're running this on.

			// Skipped.
//			CollectionResourceSize => rpc::resp::CollectionResourceSize,
//			"",
//			"",

			DaemonConfig => rpc::resp::DaemonConfig,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "ip": "127.0.0.1",
    "port": 18425,
    "max_connections": null,
    "exclusive_ips": [
      "127.0.0.1"
    ],
    "sleep_on_fail": 3000,
    "collection_paths": [],
    "tls": false,
    "certificate": null,
    "key": null,
    "rest": false,
    "docs": false,
    "direct_download": false,
    "filename_separator": " - ",
    "log_level": "OFF",
    "watch": false,
    "cache_clean": true,
    "cache_time": 3600,
    "restore_audio_state": false,
    "previous_threshold": 3,
    "media_controls": false,
    "authorization": false,
    "confirm_no_tls_auth": false,
    "no_auth_rpc": [],
    "no_auth_rest": [],
    "no_auth_docs": false
  },
  "id": 0
}"#,

			DaemonMethods => rpc::resp::DaemonMethods,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 109,
    "methods": [
      "collection_new",
      "collection_brief",
      "collection_full",
      "collection_brief_artists",
      "collection_brief_albums",
      "collection_brief_songs",
      "collection_full_artists",
      "collection_full_albums",
      "collection_full_songs",
      "collection_entries",
      "collection_perf",
      "collection_health",
      "collection_resource_size",
      "daemon_config",
      "daemon_methods",
      "daemon_no_auth_rpc",
      "daemon_no_auth_rest",
      "daemon_remove_cache",
      "daemon_save",
      "daemon_seen_ips",
      "daemon_shutdown",
      "daemon_state",
      "state_audio",
      "state_queue_key",
      "state_queue_song",
      "state_queue_entry",
      "state_playing",
      "state_repeat",
      "state_runtime",
      "state_volume",
      "key_artist",
      "key_album",
      "key_song",
      "key_entry",
      "key_artist_albums",
      "key_artist_songs",
      "key_artist_entries",
      "key_album_artist",
      "key_album_songs",
      "key_album_entries",
      "key_song_artist",
      "key_song_album",
      "key_other_albums",
      "key_other_songs",
      "key_other_entries",
      "map_artist",
      "map_album",
      "map_song",
      "map_entry",
      "map_artist_albums",
      "map_artist_songs",
      "map_artist_entries",
      "map_album_songs",
      "map_album_entries",
      "current_artist",
      "current_album",
      "current_song",
      "current_entry",
      "rand_artist",
      "rand_album",
      "rand_song",
      "rand_entry",
      "search",
      "search_artist",
      "search_album",
      "search_song",
      "search_entry",
      "toggle",
      "play",
      "pause",
      "next",
      "stop",
      "previous",
      "clear",
      "seek",
      "skip",
      "back",
      "shuffle",
      "repeat",
      "volume",
      "volume_up",
      "volume_down",
      "queue_add_key_artist",
      "queue_add_key_album",
      "queue_add_key_song",
      "queue_add_map_artist",
      "queue_add_map_album",
      "queue_add_map_song",
      "queue_add_rand_artist",
      "queue_add_rand_album",
      "queue_add_rand_song",
      "queue_add_rand_entry",
      "queue_add_playlist",
      "queue_set_index",
      "queue_remove_range",
      "playlist_new",
      "playlist_remove",
      "playlist_clone",
      "playlist_get_index",
      "playlist_remove_index",
      "playlist_add_key_artist",
      "playlist_add_key_album",
      "playlist_add_key_song",
      "playlist_add_map_artist",
      "playlist_add_map_album",
      "playlist_add_map_song",
      "playlist_single",
      "playlist_brief",
      "playlist_full"
    ]
  },
  "id": 0
}"#,

			DaemonNoAuthRpc => rpc::resp::DaemonNoAuthRpc,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "rpc": []
  },
  "id": 0
}"#,

			DaemonNoAuthRest => rpc::resp::DaemonNoAuthRest,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "rest": []
  },
  "id": 0
}"#,

			DaemonRemoveCache => rpc::resp::DaemonRemoveCache,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": [],
  "id": 0
}"#,

			DaemonSave => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			DaemonSeenIps => rpc::resp::DaemonSeenIps,
			"",
			"",

			DaemonState => rpc::resp::DaemonState,
			"",
			"", // Contains variable data, not reliable on CI.

			StateAudio => rpc::resp::StateAudio,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "queue": [],
    "queue_len": 0,
    "queue_idx": null,
    "playing": false,
    "song_key": null,
    "elapsed": 0,
    "runtime": 0,
    "repeat": "off",
    "volume": 25,
    "song": null
  },
  "id": 0
}"#,

			StateQueueKey => rpc::resp::StateQueueKey,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "keys": []
  },
  "id": 0
}"#,

			StateQueueSong => rpc::resp::StateQueueSong,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "songs": []
  },
  "id": 0
}"#,

			StateQueueEntry => rpc::resp::StateQueueEntry,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "entries": []
  },
  "id": 0
}"#,

			StatePlaying => rpc::resp::StatePlaying,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "playing": false
  },
  "id": 0
}"#,

			StateRepeat => rpc::resp::StateRepeat,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "mode": "off"
  },
  "id": 0
}"#,

			StateRuntime => rpc::resp::StateRuntime,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "elapsed": 0,
    "runtime": 0,
    "elapsed_readable": "0:00",
    "runtime_readable": "0:00"
  },
  "id": 0
}"#,

			StateVolume => rpc::resp::StateVolume,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "volume": 25
  },
  "id": 0
}"#,

			KeyArtist => rpc::resp::KeyArtist,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "artist_1",
      "key": 0,
      "runtime": 4,
      "albums": [
        0,
        1
      ],
      "songs": [
        0,
        1,
        2,
        3
      ]
    }
  },
  "id": 0
}"#,

			KeyAlbum => rpc::resp::KeyAlbum,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "album_1",
      "key": 0,
      "artist": 0,
      "release": "2018-04-25",
      "runtime": 2,
      "song_count": 2,
      "songs": [
        0,
        1
      ],
      "discs": 0,
      "art": null,
      "genre": null
    }
  },
  "id": 0
}"#,

			KeySong => rpc::resp::KeySong,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "song": {
      "title": "mp3",
      "key": 0,
      "album": 0,
      "runtime": 1,
      "sample_rate": 48000,
      "track": 1,
      "disc": null,
      "mime": "",
      "extension": ""
    }
  },
  "id": 0
}"#,

			KeyEntry => rpc::resp::KeyEntry,
			ureq::json!({"key":0}),
			"", // Skipped, don't know full PATH.

			KeyArtistAlbums => rpc::resp::KeyArtistAlbums,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ]
  },
  "id": 0
}"#,

			KeyArtistSongs => rpc::resp::KeyArtistSongs,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 4,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			KeyArtistEntries => rpc::resp::KeyArtistEntries,
			ureq::json!({"key":0}),
			"", // Skip

			KeyAlbumArtist => rpc::resp::KeyAlbumArtist,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "artist_1",
      "key": 0,
      "runtime": 4,
      "albums": [
        0,
        1
      ],
      "songs": [
        0,
        1,
        2,
        3
      ]
    }
  },
  "id": 0
}"#,

			KeyAlbumSongs => rpc::resp::KeyAlbumSongs,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			KeyAlbumEntries => rpc::resp::KeyAlbumEntries,
			ureq::json!({"key":0}),
			r#""#, // Skip

			KeySongArtist => rpc::resp::KeySongArtist,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "artist_1",
      "key": 0,
      "runtime": 4,
      "albums": [
        0,
        1
      ],
      "songs": [
        0,
        1,
        2,
        3
      ]
    }
  },
  "id": 0
}"#,

			KeySongAlbum => rpc::resp::KeySongAlbum,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "album_1",
      "key": 0,
      "artist": 0,
      "release": "2018-04-25",
      "runtime": 2,
      "song_count": 2,
      "songs": [
        0,
        1
      ],
      "discs": 0,
      "art": null,
      "genre": null
    }
  },
  "id": 0
}"#,

			KeyOtherAlbums => rpc::resp::KeyOtherAlbums,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ]
  },
  "id": 0
}"#,

			KeyOtherSongs => rpc::resp::KeyOtherSongs,
			ureq::json!({"key":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			KeyOtherEntries => rpc::resp::KeyOtherEntries,
			ureq::json!({"key":0}),
			r#""#, // Skip

			MapArtist => rpc::resp::MapArtist,
			ureq::json!({"artist":"artist_1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artist": {
      "name": "artist_1",
      "key": 0,
      "runtime": 4,
      "albums": [
        0,
        1
      ],
      "songs": [
        0,
        1,
        2,
        3
      ]
    }
  },
  "id": 0
}"#,

			MapAlbum => rpc::resp::MapAlbum,
			ureq::json!({"artist":"artist_1","album":"album_1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "album": {
      "title": "album_1",
      "key": 0,
      "artist": 0,
      "release": "2018-04-25",
      "runtime": 2,
      "song_count": 2,
      "songs": [
        0,
        1
      ],
      "discs": 0,
      "art": null,
      "genre": null
    }
  },
  "id": 0
}"#,

			MapSong => rpc::resp::MapSong,
			ureq::json!({"artist":"artist_1","album":"album_1","song":"mp3"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "song": {
      "title": "mp3",
      "key": 1,
      "album": 0,
      "runtime": 1,
      "sample_rate": 48000,
      "track": 2,
      "disc": null,
      "mime": "",
      "extension": ""
    }
  },
  "id": 0
}"#,

			MapEntry => rpc::resp::MapEntry,
			ureq::json!({"artist":"artist_1","album":"album_1","song":"mp3"}),
			r#""#, // Skip

			MapArtistAlbums => rpc::resp::MapArtistAlbums,
			ureq::json!({"artist":"artist_1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      },
      {
        "title": "album_2",
        "key": 1,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          2,
          3
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ]
  },
  "id": 0
}"#,

			MapArtistSongs => rpc::resp::MapArtistSongs,
			ureq::json!({"artist":"artist_1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 4,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 2,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			MapArtistEntries => rpc::resp::MapArtistEntries,
			ureq::json!({"artist":"artist_1","album":"album_1","song":"mp3"}),
			r#""#, // Skip

			MapAlbumSongs => rpc::resp::MapAlbumSongs,
			ureq::json!({"artist":"artist_1","album":"album_1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 2,
    "songs": [
      {
        "title": "mp3",
        "key": 0,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 1,
        "disc": null,
        "mime": "",
        "extension": ""
      },
      {
        "title": "mp3",
        "key": 1,
        "album": 0,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			MapAlbumEntries => rpc::resp::MapAlbumEntries,
			ureq::json!({"artist":"artist_1","album":"album_1"}),
			r#""#, // Skip

			// Skipped, nothing is set so these
			// will all be an `error`.
			//
//			CurrentArtist => rpc::resp::CurrentArtist,
//			CurrentAlbum => rpc::resp::CurrentAlbum,
//			CurrentSong => rpc::resp::CurrentSong,
//			CurrentEntry => rpc::resp::CurrentEntry,

			RandArtist => rpc::resp::RandArtist,
			"",
			"",

			RandAlbum => rpc::resp::RandAlbum,
			"",
			"",

			RandSong => rpc::resp::RandSong,
			"",
			"",

			RandEntry => rpc::resp::RandEntry,
			"",
			"",

			Search => rpc::resp::Search,
			ureq::json!({"input":"flac","kind":"top1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "artist_1",
        "key": 0,
        "runtime": 4,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          2,
          3
        ]
      }
    ],
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ],
    "songs": [
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			SearchArtist => rpc::resp::SearchArtist,
			ureq::json!({"input":"artist_1","kind":"top1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "artists": [
      {
        "name": "artist_1",
        "key": 0,
        "runtime": 4,
        "albums": [
          0,
          1
        ],
        "songs": [
          0,
          1,
          2,
          3
        ]
      }
    ]
  },
  "id": 0
}"#,

			SearchAlbum => rpc::resp::SearchAlbum,
			ureq::json!({"input":"album_1","kind":"top1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "albums": [
      {
        "title": "album_1",
        "key": 0,
        "artist": 0,
        "release": "2018-04-25",
        "runtime": 2,
        "song_count": 2,
        "songs": [
          0,
          1
        ],
        "discs": 0,
        "art": null,
        "genre": null
      }
    ]
  },
  "id": 0
}"#,

			SearchSong => rpc::resp::SearchSong,
			ureq::json!({"input":"flac","kind":"top1"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "songs": [
      {
        "title": "flac",
        "key": 3,
        "album": 1,
        "runtime": 1,
        "sample_rate": 48000,
        "track": 2,
        "disc": null,
        "mime": "",
        "extension": ""
      }
    ]
  },
  "id": 0
}"#,

			SearchEntry => rpc::resp::SearchEntry,
			ureq::json!({"input":"flac","kind":"top1"}),
			r#""#,

			Toggle => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Play => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Pause => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Clear => rpc::resp::Clear,
			ureq::json!({"playback":false}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0
  },
  "id": 0
}"#,

			Stop => rpc::resp::Stop,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0
  },
  "id": 0
}"#,

			Next => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Previous => rpc::resp::Status,
			ureq::json!({"threshold":0}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Skip => rpc::resp::Status,
			ureq::json!({"skip":0}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Back => rpc::resp::Status,
			ureq::json!({"back":0}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Seek => rpc::resp::Status,
			ureq::json!({"kind":"absolute","second":0}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Shuffle => rpc::resp::Status,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			Repeat => rpc::resp::Repeat,
			ureq::json!({"mode":"queue"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "previous": "off",
    "current": "queue"
  },
  "id": 0
}"#,

			// Volume + Queue + Any other operation that depends on `shukusai::audio::Audio`.
			//
			// `Audio` is going to be stuck in an infinitely loop trying to get a handle
			// to the audio output device. In CI where there is none, it will loop forever.
			// `Audio` _also_ handles signals like volume and queue mutation, so even though
			// we send these signals and `shukusai`/`Kernel` will accept them, the state
			// will not change in CI.
			//
			// So, ignore the output.
			Volume => rpc::resp::Volume,
			ureq::json!({"volume":50}),
			"",

			VolumeUp => rpc::resp::VolumeUp,
			ureq::json!({"up":5}),
			"",

			VolumeDown => rpc::resp::VolumeDown,
			ureq::json!({"down":5}),
			"",

			QueueAddKeyArtist => rpc::resp::Status,
			ureq::json!({"key":0,"append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddKeyAlbum => rpc::resp::Status,
			ureq::json!({"key":0,"append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddKeySong => rpc::resp::Status,
			ureq::json!({"key":0,"append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddMapArtist => rpc::resp::Status,
			ureq::json!({"artist":"artist_1","append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddMapAlbum => rpc::resp::Status,
			ureq::json!({"artist":"artist_1","album":"album_1","append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddMapSong => rpc::resp::Status,
			ureq::json!({"artist":"artist_1","album":"album_1","song":"mp3","append":"back","clear":false,"play":false}),
r#"{
  "jsonrpc": "2.0",
  "result": null,
  "id": 0
}"#,

			QueueAddRandArtist => rpc::resp::QueueAddRandArtist,
			ureq::json!({"append":"back","clear":false,"play":false}),
			"",

			QueueAddRandAlbum => rpc::resp::QueueAddRandAlbum,
			ureq::json!({"append":"back","clear":false,"play":false}),
			"",

			QueueAddRandSong => rpc::resp::QueueAddRandSong,
			ureq::json!({"append":"back","clear":false,"play":false}),
			"",

			QueueAddRandEntry => rpc::resp::QueueAddRandEntry,
			ureq::json!({"append":"back","clear":false,"play":false}),
			"",

			// Skipped, no playlists object.
//			QueueAddPlaylist => rpc::resp::Status,
//			ureq::json!({"append":"back","clear":false,"play":false}),
//			"",

			QueueSetIndex => rpc::resp::QueueSetIndex,
			ureq::json!({"index":0}),
			"",

			QueueRemoveRange => rpc::resp::QueueRemoveRange,
			ureq::json!({"start":0,"end":1,"skip":false}),
			"",

			// Playlists.
			//
			// Unlike `Audio`/`Queue`, the playlists are directly
			// mutatated by `festivald` via locks, so their state
			// will actually get mutated even though `Audio` is
			// stuck in an infinite loop.
			PlaylistNew => rpc::resp::PlaylistNew,
			ureq::json!({"playlist":"hello"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": null,
    "entries": null
  },
  "id": 0
}"#,

			PlaylistClone => rpc::resp::PlaylistClone,
			ureq::json!({"from":"hello","to":"hello2"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": null,
    "entries": null
  },
  "id": 0
}"#,

			PlaylistRemove => rpc::resp::PlaylistRemove,
			ureq::json!({"playlist":"hello2"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 0,
    "entries": []
  },
  "id": 0
}"#,

			PlaylistAddKeyArtist => rpc::resp::PlaylistAddKeyArtist,
			ureq::json!({"key":0,"playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 0,
    "new_len": 4
  },
  "id": 0
}"#,

			PlaylistAddKeyAlbum => rpc::resp::PlaylistAddKeyAlbum,
			ureq::json!({"key":0,"playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 4,
    "new_len": 6
  },
  "id": 0
}"#,

			PlaylistAddKeySong => rpc::resp::PlaylistAddKeySong,
			ureq::json!({"key":0,"playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 6,
    "new_len": 7
  },
  "id": 0
}"#,

			PlaylistAddMapArtist => rpc::resp::PlaylistAddMapArtist,
			ureq::json!({"artist":"artist_1","playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 7,
    "new_len": 11
  },
  "id": 0
}"#,

			PlaylistAddMapAlbum => rpc::resp::PlaylistAddMapAlbum,
			ureq::json!({"artist":"artist_1","album":"album_1","playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 11,
    "new_len": 13
  },
  "id": 0
}"#,

			PlaylistAddMapSong => rpc::resp::PlaylistAddMapSong,
			ureq::json!({"artist":"artist_1","album":"album_1","song":"mp3","playlist":"hello","append":"back"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "existed": true,
    "old_len": 13,
    "new_len": 14
  },
  "id": 0
}"#,

			PlaylistSingle => rpc::resp::PlaylistSingle,
			ureq::json!({"playlist":"hello"}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "playlist": "hello",
    "all_valid": true,
    "entry_len": 14,
    "valid_len": 14,
    "invalid_len": 0,
    "entries": [
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 0,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 1,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 1,
          "key_song": 2,
          "artist": "artist_1",
          "album": "album_2",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 1,
          "key_song": 3,
          "artist": "artist_1",
          "album": "album_2",
          "song": "flac"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 0,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 1,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 0,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 0,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 1,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 1,
          "key_song": 2,
          "artist": "artist_1",
          "album": "album_2",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 1,
          "key_song": 3,
          "artist": "artist_1",
          "album": "album_2",
          "song": "flac"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 0,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 1,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      },
      {
        "valid": {
          "key_artist": 0,
          "key_album": 0,
          "key_song": 1,
          "artist": "artist_1",
          "album": "album_1",
          "song": "mp3"
        }
      }
    ]
  },
  "id": 0
}"#,

			PlaylistBrief => rpc::resp::PlaylistBrief,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "len": 1,
    "playlists": [
      "hello"
    ]
  },
  "id": 0
}"#,

			PlaylistFull => rpc::resp::PlaylistFull,
			"",
r#"{
  "jsonrpc": "2.0",
  "result": {
    "all_valid": true,
    "playlist_len": 1,
    "entry_len": 14,
    "valid_len": 14,
    "invalid_len": 0,
    "playlists": {
      "hello": [
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 0,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 1,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 1,
            "key_song": 2,
            "artist": "artist_1",
            "album": "album_2",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 1,
            "key_song": 3,
            "artist": "artist_1",
            "album": "album_2",
            "song": "flac"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 0,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 1,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 0,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 0,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 1,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 1,
            "key_song": 2,
            "artist": "artist_1",
            "album": "album_2",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 1,
            "key_song": 3,
            "artist": "artist_1",
            "album": "album_2",
            "song": "flac"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 0,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 1,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        },
        {
          "valid": {
            "key_artist": 0,
            "key_album": 0,
            "key_song": 1,
            "artist": "artist_1",
            "album": "album_1",
            "song": "mp3"
          }
        }
      ]
    }
  },
  "id": 0
}"#,

			PlaylistGetIndex => rpc::resp::PlaylistGetIndex,
			ureq::json!({"playlist":"hello","index":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "valid": {
        "key_artist": 0,
        "key_album": 0,
        "key_song": 0,
        "artist": "artist_1",
        "album": "album_1",
        "song": "mp3"
      }
    }
  },
  "id": 0
}"#,

			PlaylistRemoveIndex => rpc::resp::PlaylistRemoveIndex,
			ureq::json!({"playlist":"hello","index":0}),
r#"{
  "jsonrpc": "2.0",
  "result": {
    "entry": {
      "valid": {
        "key_artist": 0,
        "key_album": 0,
        "key_song": 0,
        "artist": "artist_1",
        "album": "album_1",
        "song": "mp3"
      }
    }
  },
  "id": 0
}"#,

			// Saved until last.
			DaemonShutdown => rpc::resp::DaemonShutdown,
			"",
			"" // Skipped, contains variable data.
		}
	}
}
