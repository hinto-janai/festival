//---------------------------------------------------------------------------------------------------- Use
use crate::ccd::msg::CcdToKernel;
use crate::collection::{Album, Art, Collection};
use benri::{log::*, sync::*};
use crossbeam::channel::Sender;
use log::warn;
use rayon::prelude::*;

//---------------------------------------------------------------------------------------------------- Types of conversions
pub(super) enum ArtConvertType {
    // The user requested a new `Collection`,
    // and this conversion is part of a bigger reset.
    // This resizes existing `Art::Bytes`.
    Resize,

    // We're converting `Art::Bytes` -> `Art::Known`.
    ToKnown,
}

//---------------------------------------------------------------------------------------------------- Conversion (bytes <-> egui image) functions
impl super::Ccd {
    #[inline(always)]
    // Internal re-usable image conversion function.
    // This can be used in `new_collection()` as well.
    pub(super) fn priv_convert_art(
        to_kernel: &Sender<CcdToKernel>,
        collection: &mut Collection,
        art_convert_type: ArtConvertType,
        increment: f64,
    ) {
        match art_convert_type {
            ArtConvertType::Resize => {
                collection
                    .albums
                    .0
                    .par_iter_mut()
                    .for_each(|album| Self::resize(to_kernel, album, increment));
            }
            ArtConvertType::ToKnown => {
                collection
                    .albums
                    .0
                    .par_iter_mut()
                    .for_each(|album| Self::known(to_kernel, album, increment));
            }
        }
    }

    #[inline(always)]
    // The actual art conversion "processing" work.
    // This is for `ArtConvertType::Resize`.
    pub(super) fn resize(to_kernel: &Sender<CcdToKernel>, album: &mut Album, increment: f64) {
        // Resizer.
        let mut resizer = crate::ccd::create_resizer();

        send!(
            to_kernel,
            CcdToKernel::UpdateIncrement((increment, album.title.clone()))
        );

        // If bytes exist, convert, else provide the `Unknown` art.
        album.art = match &mut album.art {
            Art::Bytes(b) => {
                ok_trace!("{}", album.title);

                let b = std::mem::take(b);

                match super::art_from_raw(b, &mut resizer) {
                    Ok(b) => Art::Bytes(b),
                    Err(e) => {
                        warn!("Art error: {e} ... {}", album.title);
                        Art::Unknown
                    }
                }
            }
            _ => {
                skip_trace!("{}", album.title);
                Art::Unknown
            }
        };
    }

    #[inline(always)]
    // This is for `ArtConvertType::ToKnown`.
    pub(super) fn known(to_kernel: &Sender<CcdToKernel>, album: &mut Album, increment: f64) {
        send!(
            to_kernel,
            CcdToKernel::UpdateIncrement((increment, album.title.clone()))
        );

        // If bytes exist, convert, else provide the `Unknown` art.
        album.art = match &mut album.art {
            Art::Bytes(b) => {
                ok_trace!("{}", album.title);
                let b = std::mem::take(b);
                Art::Known(super::art_from_known(b))
            }
            _ => {
                skip_trace!("{}", album.title);
                Art::Unknown
            }
        };
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
