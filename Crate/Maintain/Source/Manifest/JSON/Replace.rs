//! The first `"version": "..."` pair, rewritten in place.

/// Rewrite the first `"version": "..."` pair, but only when the value found
/// there is the one JSON parsing reported. Without that check a nested
/// `version` elsewhere in the file would be silently edited instead.
///
/// The text is spliced rather than re-serialised because `serde_json` does not
/// preserve formatting: a round trip would reflow the whole manifest to change
/// one line.
pub fn Fn(Text:&str, Expected:&str, Version:&str) -> Option<String> {
	let Key = "\"version\"";
	let KeyAt = Text.find(Key)?;
	let AfterKey = &Text[KeyAt + Key.len()..];
	let ColonAt = AfterKey.find(':')?;
	let AfterColon = &AfterKey[ColonAt + 1..];
	let OpenAt = AfterColon.find('"')?;
	let ValueAt = OpenAt + 1;
	let CloseAt = ValueAt + AfterColon[ValueAt..].find('"')?;
	let Found = &AfterColon[ValueAt..CloseAt];

	(Found == Expected).then(|| {
		let Start = KeyAt + Key.len() + ColonAt + 1 + ValueAt;

		format!("{}{}{}", &Text[..Start], Version, &Text[Start + Found.len()..])
	})
}
