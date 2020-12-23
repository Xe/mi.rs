pub mod types;
pub use types::SiteData;

/// The credit string for this data.
///
/// XXX(acli): the license [here](https://dd.weather.gc.ca/doc/LICENCE_GENERAL.txt)
/// demands that we include this string somewhere in data derived from this API.
/// This must be manually included in each response to remain within the scope
/// of the license.
pub const DATA_SOURCE: &'static str = "Data Source: Environment and Climate Change Canada";
