//---------------------------------------------------------------------------------------------------- Use
use crate::data::{DebugInfo, Settings, State, StateRestore};
use crossbeam::channel::{Receiver, Sender};
use shukusai::{
    collection::{AlbumKey, ArtistKey, Collection, KeyEnum, SongKey},
    kernel::{FrontendToKernel, KernelToFrontend},
    state::{AudioState, ResetState},
};
use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::time::Instant;

//---------------------------------------------------------------------------------------------------- GUI struct. This holds ALL data.
pub struct Gui {
    //-------------------------------------------------- Long-Term State.
    // These represent state that pretty much
    // have a static lifetime and are used everywhere,
    // and is expected to always exist in some form.
    /// To `Kernel`.
    pub to_kernel: Sender<FrontendToKernel>,
    /// From `Kernel`.
    pub from_kernel: Receiver<KernelToFrontend>,

    /// The `Collection`.
    pub collection: Arc<Collection>,

    /// `GUI` settings.
    pub settings: Settings,
    /// `GUI` settings (old).
    pub og_settings: Settings,

    /// `GUI` state.
    pub state: State,
    /// `GUI` settings (old).
    pub og_state: State,
    /// In-between `Collection`-reset state to be potentially carried over.
    pub state_restore: StateRestore,

    //-------------------------------------------------- Ephemeral State.
    // These are quick one-off fields that mostly
    // act as flags or some small thing for
    // functionality in the GUI.
    /// Audio State.
    ///
    /// A local copy of shukusai's `AUDIO_STATE`
    /// so we don't have to lock it multiple times every loop.
    pub audio_state: AudioState,
    /// Total runtime (time) of the queue.
    pub queue_time: readable::Time,
    /// A local copy of our seek time.
    /// This is the thing we send `Kernel` when
    /// we want to seek the audio.
    pub audio_seek: u64,
    /// Since we copy `AUDIO_STATE`'s audio time every loop,
    /// the slider will bounce back and flicker during the
    /// few frames we are waiting for our message to pass
    /// to `Kernel`, then to `Audio`, so create this signal
    /// so we know we shouldn't overwrite just yet.
    pub audio_leeway: Instant,
    /// The [`SongKey`] we were playing in the last frame.
    pub last_song: Option<SongKey>,
    /// The pixel size needed for the `Runtime` in the bottom UI bar.
    /// It depends on the length of the `Runtime` string.
    pub runtime_width: f32,
    /// AudioState auto-save interval.
    pub auto_save: Instant,

    /// Reset State.
    ///
    /// A local copy of shukusai's `RESET_STATE`
    /// so we don't have to lock it multiple times every loop.
    pub reset_state: ResetState,

    /// Our max `rect` at the very start of every frame.
    pub rect: egui::Rect,
    /// When using CTRL+(LEFT|RIGHT|UP|DOWN) to pin the window
    /// to a side or minimize/maximize, egui is a little too
    /// eager and registers the left/right/up/down buttons.
    ///
    /// Whenever we resize, this `Instant` will be set so that
    /// we can have a little leeway when registering key presses.
    pub resize_leeway: Instant,

    /// `egui_notify` state.
    pub toasts: egui_notify::Toasts,

    // `RFD` state.
    /// If a RFD window is currently open.
    pub rfd_open: Arc<AtomicBool>,
    /// If a file was selected with RFD.
    pub rfd_new: Arc<Mutex<Option<PathBuf>>>,
    /// A buffer of the indices of the PATHs the user wants deleted.
    pub deleted_paths: Vec<usize>,

    // `Search` state.
    /// If we're currently searching.
    pub searching: bool,
    /// If the user types [A-Za-z0-9] from anywhere,
    /// we switch to the `Search` tab, input the
    /// `String` and set this [`bool`] so that
    /// the GUI knows to `request_focus()` the search `TextEdit`.
    pub search_jump: bool,

    // Playlist state.
    /// Originl playlist copy.
    pub og_playlists: shukusai::state::Playlists,
    /// Clone this playlist ASAP
    pub playlist_clone: Option<Arc<str>>,
    /// Remove this playlist ASAP
    pub playlist_remove: Option<Arc<str>>,
    /// Edit this name...
    pub playlist_from: Option<Arc<str>>,
    /// To this name.
    pub playlist_to: Option<Arc<str>>,
    /// Move the position of these entries in this playlist.
    pub playlist_swap_entry: Option<(Arc<str>, usize, usize)>,
    /// Remove the entry at this index from this playlist.
    pub playlist_remove_entry: Option<(Arc<str>, usize)>,
    /// A playlist name edit had a `\n` enter, so we should save.
    pub playlist_name_edit_enter: bool,
    /// Are we adding a `Artist/Album/Song` to a playlist?
    /// (fullscreen menu)
    /// This holds the keys that we should add.
    pub playlist_add_screen: Option<KeyEnum>,
    /// Add these keys to this playlist ASAP
    pub playlist_add_screen_result: Option<(Arc<str>, KeyEnum)>,

    // Local cached variables.
    /// A cached, formatted version of [`Collection::count_artist`]
    pub count_artist: String,
    /// A cached, formatted version of [`Collection::count_album`]
    pub count_album: String,
    /// A cached, formatted version of [`Collection::count_song`]
    pub count_song: String,
    /// A cached, formatted version of [`Collection::count_art`]
    pub count_art: String,

    // Exit state.
    /// Are we currently in the process of exiting?
    pub exiting: bool,
    /// To prevent showing a flash of the spinner
    /// when exiting really quickly, this `Instant`
    /// needs to rack up some time before showing the spinner.
    pub exit_instant: Instant,

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

    /// A local copy of `egui::Modifier` so we don't have to lock
    /// `Context` every time we need access to this frames data.
    pub modifiers: egui::Modifiers,
}
