//---------------------------------------------------------------------------------------------------- Use
use crate::constants::{FESTIVAL_VERSION, GUI};
use crate::data::{exit::EXIT_COUNTDOWN, gui::Gui};
use benri::atomic_load;
use const_format::formatcp;
use serde::{Deserialize, Serialize};
use shukusai::{
    constants::{BUILD, COMMIT, DASH, FESTIVAL, SHUKUSAI_VERSION, TXT_SUB_DIR},
    logger::INIT_INSTANT,
    state::AUDIO_STATE,
    thread::THREADS,
};

//---------------------------------------------------------------------------------------------------- Debug screen formatter.
disk::plain!(
    DebugInfo,
    disk::Dir::Data,
    FESTIVAL,
    formatcp!("{GUI}/{TXT_SUB_DIR}"),
    "debug.txt"
);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
impl Gui {
    pub fn update_debug_info(&mut self) {
        let info = format!(
            "{DASH} sys
os      | {} {}
args    | {:?}
threads | {}
elapsed | {} seconds

{DASH} festival
build    | {}
commit   | {}
version  | {}
shukusai | {}
resample | {}

{DASH} diff
state    | {}
settings | {}

{DASH} rfd
rfd_open | {}

{DASH} search
searching     | {}
search_string | {:?}

{DASH} cache
count_artist | {}
count_album  | {}
count_song   | {}
count_art    | {}

{DASH} exit
exiting        | {}
exit_countdown | {}

{DASH} collection
resetting_collection | {}
kernel_returned      | {}

{DASH} audio state
{:#?}

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
            *THREADS,
            INIT_INSTANT.elapsed().as_secs_f64(),
            BUILD,
            COMMIT,
            FESTIVAL_VERSION,
            SHUKUSAI_VERSION,
            std::env::var_os("FESTIVAL_FORCE_RESAMPLE").is_some(),
            self.diff_state(),
            self.diff_settings(),
            atomic_load!(self.rfd_open),
            self.searching,
            self.state.search_string,
            self.count_artist,
            self.count_album,
            self.count_song,
            self.count_art,
            self.exiting,
            atomic_load!(EXIT_COUNTDOWN),
            self.resetting_collection,
            self.kernel_returned,
            AUDIO_STATE.read(),
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
