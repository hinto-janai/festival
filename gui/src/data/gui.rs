//---------------------------------------------------------------------------------------------------- Use
use crate::{
	constants::*,
};
use crate::data::{
	State,
	Settings,
};
use super::{
	DebugInfo,
	Tab,
};
use shukusai::kernel::{
	Kernel,
	KernelState,
	ResetState,
	FrontendToKernel,
	KernelToFrontend,
};
use shukusai::collection::{
	Collection,
	AlbumKey,
	Keychain,
};
use shukusai::sort::{
	ArtistSort,AlbumSort,SongSort,
};
use benri::{
	now,
	debug_panic,
	log::*,
	panic::*,
	sync::*,
};
use log::{
	info,
	warn,
	error
};
use egui::{
	Style,Visuals,Color32,
	TopBottomPanel,SidePanel,CentralPanel,
	TextStyle,FontId,FontData,FontDefinitions,FontFamily,FontTweak,
};
use crossbeam::channel::{Sender,Receiver};
use std::path::PathBuf;
use std::sync::{
	Arc,
	Mutex,
	atomic::AtomicBool,
	atomic::AtomicU8,
};
use rolock::RoLock;
use disk::{Bincode2,Toml,Json};
use std::time::Instant;
use super::AlbumSizing;

//---------------------------------------------------------------------------------------------------- GUI struct. This hold ALL data.
pub struct Gui {
	/// To `Kernel`.
	pub to_kernel: Sender<FrontendToKernel>,
	/// From `Kernel`.
	pub from_kernel: Receiver<KernelToFrontend>,

	/// The `Collection` and misc state.
	pub collection: Arc<Collection>,
	pub kernel_state: RoLock<KernelState>,
	pub reset_state: RoLock<ResetState>,

	/// `GUI` settings.
	pub settings: Settings,
	/// `GUI` settings (old).
	pub og_settings: Settings,

	/// `GUI` state.
	pub state: State,
	/// `GUI` settings (old).
	pub og_state: State,

	// RFD state.
	/// If a RFD window is currently open.
	pub rfd_open: Arc<AtomicBool>,
	/// If a file was selected with RFD.
	pub rfd_new: Arc<Mutex<Option<PathBuf>>>,
	/// A buffer of the indices of the PATHs the user wants deleted.
	pub deleted_paths: Vec<usize>,

	// Search state.
	/// If we're currently searching.
	pub searching: bool,
	/// If the user types English from anywhere,
	/// we switch to the `Search` tab, input the
	/// `String` and set this [`bool`] so that
	/// the GUI knows to `request_focus()` the search `TextEdit`.
	pub search_focus: bool,
	/// Our current search input.
	pub search_string: String,
	/// The search result [`Keychain`] we got from `Kernel`.
	pub search_result: Keychain,

	// Local cached variables.
	/// A cached, formatted version of [`Collection::count_artist`]
	pub count_artist: String,
	/// A cached, formatted version of [`Collection::count_album`]
	pub count_album: String,
	/// A cached, formatted version of [`Collection::count_song`]
	pub count_song: String,

	// Exit state.
	/// Are we currently in the process of exiting?
	pub exiting: bool,
	/// To prevent showing a flash of the spinner
	/// when exiting really quickly, this `Instant`
	/// needs to rack up some time before showing the spinner.
	pub exit_instant: Instant,
	/// How long before we force quit without saving.
	pub exit_countdown: Arc<AtomicU8>,

	// Reset/Collection state.
	/// Are we in the middle of resetting the [`Collection`]?
	pub resetting_collection: bool,
	/// This is a [`bool`] that is `false` until `Kernel`
	/// responds with any message after it's done startup.
	///
	/// Once we get our first message from `Kernel`, this
	/// will always be `true`. This is used for things like
	/// the initial album art spinner screen.
	pub kernel_returned: bool,

	// Debug screen.
	/// Are we showing the debug screen?
	///
	/// (User pressed CTRL+SHIFT+D)
	pub debug_screen: bool,
	/// The debug info displayed on the debug screen.
	pub debug_info: DebugInfo,

	// Tab history (for mouse).
	/// The last [`Tab`] the user was on.
	pub last_tab: Option<Tab>,
}
