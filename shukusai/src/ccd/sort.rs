//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{Album, AlbumKey, Artist, ArtistKey, Song, SongKey};

//---------------------------------------------------------------------------------------------------- __NAME__
// These functions create new sorted `Vec<_Key>`'s.
// Each function matches a field within the `Collection`.
//
// INVARIANT:
// These functions assume the input data is correct.
// AKA, the `Collection` should already be filled out with (un-sorted) data.
//
// They also depend on each other as it goes down, e.g:
// Songs depend on -> Sorted Albums depends on -> Sorted Artists
//
// So, these functions should be called from top to bottom as defined here, such
// that the output of the previous function can be used as the input to the next.
impl super::Ccd {
    // Returns a `Vec` filled with a specified amount of `usize`.
    fn filled_vec_usize(len: usize) -> Vec<usize> {
        (0..len).collect()
    }

    //--------------------------------------------------------------- `ArtistKey` sorts.
    pub(super) fn sort_artist_lexi(artists: &[Artist]) -> Box<[ArtistKey]> {
        let mut vec_artist = Self::filled_vec_usize(artists.len());
        vec_artist.sort_by(|a, b| {
            artists[*a]
                .name
                .to_lowercase()
                .cmp(&artists[*b].name.to_lowercase())
        });
        vec_artist.into_iter().map(ArtistKey::from).collect()
    }

    pub(super) fn sort_artist_album_count(artists: &[Artist]) -> Box<[ArtistKey]> {
        let mut vec_artist = Self::filled_vec_usize(artists.len());
        vec_artist.sort_by(|a, b| artists[*a].albums.len().cmp(&artists[*b].albums.len()));
        vec_artist.into_iter().map(ArtistKey::from).collect()
    }

    pub(super) fn sort_artist_song_count(artists: &[Artist], albums: &[Album]) -> Box<[ArtistKey]> {
        let mut vec_artist = Self::filled_vec_usize(artists.len());
        vec_artist.sort_by(|a, b| {
            let first: usize = artists[*a]
                .albums
                .iter()
                .map(|a| albums[a.inner()].songs.len())
                .sum();
            let second: usize = artists[*b]
                .albums
                .iter()
                .map(|b| albums[b.inner()].songs.len())
                .sum();

            first.cmp(&second)
        });

        vec_artist.into_iter().map(ArtistKey::from).collect()
    }

    pub(super) fn sort_artist_runtime(artists: &[Artist]) -> Box<[ArtistKey]> {
        let mut vec_artist = Self::filled_vec_usize(artists.len());

        vec_artist.sort_by(|a, b| artists[*a].runtime.cmp(&artists[*b].runtime));

        vec_artist.into_iter().map(ArtistKey::from).collect()
    }

    pub(super) fn sort_artist_name(artists: &[Artist]) -> Box<[ArtistKey]> {
        let mut vec_artist = Self::filled_vec_usize(artists.len());
        vec_artist.sort_by(|a, b| artists[*a].name.len().cmp(&artists[*b].name.len()));
        vec_artist.into_iter().map(ArtistKey::from).collect()
    }

    //--------------------------------------------------------------- `AlbumKey` sorts.
    // INVARIANT:
    // These album functions require an already lexi-sorted `Vec<ArtistKey>`
    // since this iterates over the artists, and gets their albums along the way.
    pub(super) fn sort_album_release_artist_iter(
        sorted_artists: &[ArtistKey],
        artists: &[Artist],
        albums: &[Album],
    ) -> Box<[AlbumKey]> {
        let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

        for artist in sorted_artists {
            let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
            tmp.sort_by(|a, b| albums[a.inner()].release.cmp(&albums[b.inner()].release));
            vec_album.push(tmp);
        }

        vec_album.into_iter().flatten().collect()
    }

    pub(super) fn sort_album_release_rev_artist_iter(
        sorted_artists: &[ArtistKey],
        artists: &[Artist],
        albums: &[Album],
    ) -> Box<[AlbumKey]> {
        let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

        for artist in sorted_artists {
            let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
            tmp.sort_by(|a, b| albums[a.inner()].release.cmp(&albums[b.inner()].release));
            vec_album.push(tmp.into_iter().rev().collect());
        }

        vec_album.into_iter().flatten().collect()
    }

    pub(super) fn sort_album_lexi_artist_iter(
        sorted_artists: &[ArtistKey],
        artists: &[Artist],
        albums: &[Album],
    ) -> Box<[AlbumKey]> {
        let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

        for artist in sorted_artists {
            let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
            tmp.sort_by(|a, b| {
                albums[a.inner()]
                    .title
                    .to_lowercase()
                    .cmp(&albums[b.inner()].title.to_lowercase())
            });
            vec_album.push(tmp);
        }

        vec_album.into_iter().flatten().collect()
    }

    pub(super) fn sort_album_lexi_rev_artist_iter(
        sorted_artists: &[ArtistKey],
        artists: &[Artist],
        albums: &[Album],
    ) -> Box<[AlbumKey]> {
        let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

        for artist in sorted_artists {
            let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
            tmp.sort_by(|a, b| {
                albums[a.inner()]
                    .title
                    .to_lowercase()
                    .cmp(&albums[b.inner()].title.to_lowercase())
            });
            vec_album.push(tmp.into_iter().rev().collect());
        }

        vec_album.into_iter().flatten().collect()
    }

    // Doesn't require `Vec<Artist>`.
    pub(super) fn sort_album_lexi(albums: &[Album]) -> Box<[AlbumKey]> {
        let mut vec_album = Self::filled_vec_usize(albums.len());

        vec_album.sort_by(|a, b| {
            albums[*a]
                .title
                .to_lowercase()
                .cmp(&albums[*b].title.to_lowercase())
        });

        vec_album.into_iter().map(AlbumKey::from).collect()
    }

    pub(super) fn sort_album_release(albums: &[Album]) -> Box<[AlbumKey]> {
        let mut vec_album = Self::filled_vec_usize(albums.len());

        vec_album.sort_by(|a, b| albums[*a].release.cmp(&albums[*b].release));

        vec_album.into_iter().map(AlbumKey::from).collect()
    }

    // INVARIANT:
    // `runtime` is a `f64` which could be `NaN`.
    // Except I (CCD) control this and it's always at least
    // initialized as `0.0` so using `cmp_f64` is fine (it ignores `NaN`s).
    pub(super) fn sort_album_runtime(albums: &[Album]) -> Box<[AlbumKey]> {
        let mut vec_album = Self::filled_vec_usize(albums.len());

        vec_album.sort_by(|a, b| albums[*a].runtime.inner().cmp(&albums[*b].runtime.inner()));

        vec_album.into_iter().map(AlbumKey::from).collect()
    }

    // INVARIANT:
    // `runtime` is a `f64` which could be `NaN`.
    // Except I (CCD) control this and it's always at least
    // initialized as `0.0` so using `cmp_f64` is fine (it ignores `NaN`s).
    pub(super) fn sort_album_title(albums: &[Album]) -> Box<[AlbumKey]> {
        let mut vec_album = Self::filled_vec_usize(albums.len());

        vec_album.sort_by(|a, b| albums[*a].title.len().cmp(&albums[*b].title.len()));

        vec_album.into_iter().map(AlbumKey::from).collect()
    }

    //--------------------------------------------------------------- `SongKey` sorts.
    // INVARIANT:
    // Needs an already sorted `Vec<AlbumKey>`.
    //
    // The ordering of the `Song`'s are just based off iterating
    // on the given `AlbumKey`'s. So whatever order the `AlbumKey`'s
    // are in, the `Song`'s will be as well.
    pub(super) fn sort_song(sorted_albums: &[AlbumKey], albums: &[Album]) -> Box<[SongKey]> {
        let vec_song: Vec<Vec<SongKey>> = sorted_albums
            .iter()
            .map(|a| albums[a.inner()].songs.clone())
            .collect();

        vec_song.into_iter().flatten().collect()
    }

    pub(super) fn sort_song_lexi(songs: &[Song]) -> Box<[SongKey]> {
        let mut vec_song = Self::filled_vec_usize(songs.len());

        vec_song.sort_by(|a, b| {
            songs[*a]
                .title
                .to_lowercase()
                .cmp(&songs[*b].title.to_lowercase())
        });

        vec_song.into_iter().map(SongKey::from).collect()
    }

    // INVARIANT:
    // `f64` must not be a `NaN`.
    // (It won't be, I control it).
    pub(super) fn sort_song_runtime(songs: &[Song]) -> Box<[SongKey]> {
        let mut vec_song = Self::filled_vec_usize(songs.len());

        vec_song.sort_by(|a, b| songs[*a].runtime.inner().cmp(&songs[*b].runtime.inner()));

        vec_song.into_iter().map(SongKey::from).collect()
    }

    // INVARIANT:
    // `f64` must not be a `NaN`.
    // (It won't be, I control it).
    pub(super) fn sort_song_title(songs: &[Song]) -> Box<[SongKey]> {
        let mut vec_song = Self::filled_vec_usize(songs.len());

        vec_song.sort_by(|a, b| songs[*a].title.len().cmp(&songs[*b].title.len()));

        vec_song.into_iter().map(SongKey::from).collect()
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
// SOMEDAY: After eyeballing my own `Collection`'s sort orders
// I'm pretty certain all the functions here (if used correctly) result
// in the correct output.
//
// I really don't want to write these tests... but someday.
//
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
