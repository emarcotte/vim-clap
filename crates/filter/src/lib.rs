//! This crate provides the feature of filtering a stream of lines.
//!
//! Given a stream of lines:
//!
//! 1. apply the matcher algorithm on each of them.
//! 2. sort the all lines with a match result.
//! 3. print the top rated filtered lines to stdout.

mod dynamic;
mod source;

use anyhow::Result;
use rayon::prelude::*;

use matcher::{Algo, Bonus, MatchType, Matcher};
use source_item::SourceItem;

pub use self::dynamic::dyn_run;
pub use self::source::Source;
pub use matcher;
#[cfg(feature = "enable_dyn")]
pub use subprocess;

/// Tuple of (matched line text, filtering score, indices of matched elements)
pub type FilterResult = (SourceItem, i64, Vec<usize>);

/// Returns the ranked results after applying the matcher algo
/// given the query String and filtering source.
pub fn sync_run<I: Iterator<Item = SourceItem>>(
    query: &str,
    source: Source<I>,
    algo: Algo,
    match_type: MatchType,
    bonuses: Vec<Bonus>,
) -> Result<Vec<FilterResult>> {
    let matcher = Matcher::new_with_bonuses(algo, match_type, bonuses);
    let mut ranked = source.filter(matcher, query)?;

    ranked.par_sort_unstable_by(|(_, v1, _), (_, v2, _)| v2.partial_cmp(&v1).unwrap());

    Ok(ranked)
}
