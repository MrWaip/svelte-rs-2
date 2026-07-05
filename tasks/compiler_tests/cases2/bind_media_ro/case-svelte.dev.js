App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<audio></audio>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let buffered = $.tag($.state(void 0), "buffered");
	let seekable = $.tag($.state(void 0), "seekable");
	let seeking = $.tag($.state(false), "seeking");
	let ended = $.tag($.state(false), "ended");
	let readyState = $.tag($.state(0), "readyState");
	let played = $.tag($.state(void 0), "played");
	var $$exports = { ...$.legacy_api() };
	var audio = root();
	$.bind_buffered(audio, function set($$value) {
		$.set(buffered, $$value);
	});
	$.bind_seekable(audio, function set($$value) {
		$.set(seekable, $$value);
	});
	$.bind_seeking(audio, function set($$value) {
		$.set(seeking, $$value);
	});
	$.bind_ended(audio, function set($$value) {
		$.set(ended, $$value);
	});
	$.bind_ready_state(audio, function set($$value) {
		$.set(readyState, $$value);
	});
	$.bind_played(audio, function set($$value) {
		$.set(played, $$value);
	});
	$.append($$anchor, audio);
	return $.pop($$exports);
}
