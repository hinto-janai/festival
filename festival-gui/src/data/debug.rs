//---------------------------------------------------------------------------------------------------- Use
use super::Gui;
use shukusai::{
	DASH,
	BUILD,
	COMMIT,
	FESTIVAL,
	FESTIVAL_NAME_VER,
};
use shukusai::{
	threads_available,
	init_instant,
};
use benri::atomic_load;
use serde::{Serialize,Deserialize};

//---------------------------------------------------------------------------------------------------- Debug screen formatter.
disk::plain!(DebugInfo, disk::Dir::Data, FESTIVAL, "gui", "debug");
#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize)]
#[serde(transparent)]
/// File representing GUI debug info.
///
/// This gets written in the `festival/gui` folder as `debug.txt`.
pub struct DebugInfo(String);

impl DebugInfo {
	pub fn new() -> Self {
		Self(String::new())
	}

	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

//---------------------------------------------------------------------------------------------------- Debug screen formatter.
impl super::Gui {
	pub fn update_debug_info(&mut self) {
		let info = format!(
"{DASH} sys
os      | {} {}
args    | {:?}
threads | {}
elapsed | {} seconds

{DASH} festival
build   | {}
commit  | {}version | {}

{DASH} diff
state    | {}
settings | {}

{DASH} rfd
rfd_open | {}

{DASH} search
searching     | {}
search_string | {}

{DASH} cache
count_artist | {}
count_album  | {}
count_song   | {}

{DASH} exit
exiting        | {}
exit_countdown | {}

{DASH} collection
resetting_collection | {}
kernel_returned      | {}

{DASH} thread
{:#?}

{DASH} state
{:#?}

{DASH} og_state
{:#?}

{DASH} settings
{:#?}

{DASH} og_settings
{:#?}

{DASH} collection (struct)
{}",
			std::env::consts::OS,
			std::env::consts::ARCH,
			std::env::args_os(),
			threads_available(),
			init_instant().elapsed().as_secs_f64(),
			BUILD,
			COMMIT,
			FESTIVAL_NAME_VER,
			self.diff_state(),
			self.diff_settings(),
			atomic_load!(self.rfd_open),
			self.searching,
			self.search_string,
			self.count_artist,
			self.count_album,
			self.count_song,
			self.exiting,
			atomic_load!(self.exit_countdown),
			self.resetting_collection,
			self.kernel_returned,
			std::thread::current(),
			self.state,
			self.og_state,
			self.settings,
			self.og_settings,
			self.collection,
		);

		self.debug_info = DebugInfo(info);
	}
}
