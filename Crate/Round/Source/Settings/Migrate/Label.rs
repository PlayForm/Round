//! Carrying a monitor's overrides onto the label it carries now.

pub mod Adopt;
pub mod Clear;
pub mod Former;

use std::collections::BTreeMap;

use crate::{Per::Struct::Struct as Per, Screen::Struct::Struct as Screen};

/// The same setting, with every override an older build stored under a monitor's
/// former label moved onto the label that monitor carries now.
///
/// A label used to be the monitor's name and nothing else, so two displays of
/// the same model were stored under one key and shared one set of overrides.
/// Both of them were drawn with that value, so both of them keep it: the carries
/// are collected against the map as it was and merged afterwards, never one
/// monitor at a time against a map the previous monitor has already emptied.
///
/// This runs on every read rather than once behind a version gate, because a
/// display that is not connected at launch has no monitor to carry its settings
/// onto. Its key stays in the store untouched, waiting for the launch it comes
/// back on - the alternative is dropping the settings of every display that
/// happened to be unplugged the first time the new build ran.
///
/// Which key a monitor's settings are under is asked through [`Former`], because
/// a display can have been stored under more than the one it carries now: its
/// bare name, its position-qualified name, or the name the release before unique
/// labels used.
pub fn Fn<T:Clone>(Value:Per<T>, Screens:&[Screen]) -> Per<T> {
	let Carried:Vec<(String, T)> = Screens.iter().filter_map(|Screen| Adopt::Fn(&Value.Monitor, Screen)).collect();

	let Adopted:BTreeMap<String, T> = Value.Monitor.into_iter().chain(Carried).collect();

	let Monitor = Screens.iter().fold(Adopted, |Monitor, Screen| Clear::Fn(Monitor, Screen));

	Per { Default:Value.Default, Monitor }
}
