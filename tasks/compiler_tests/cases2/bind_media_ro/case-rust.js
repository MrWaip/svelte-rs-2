import * as $ from "svelte/internal/client";
var root = $.from_html(`<audio></audio>`);
export default function App($$anchor) {
	let buffered = $.state(void 0);
	let seekable = $.state(void 0);
	let seeking = $.state(false);
	let ended = $.state(false);
	let readyState = $.state(0);
	let played = $.state(void 0);
	var audio = root();
	$.bind_buffered(audio, ($$value) => $.set(buffered, $$value));
	$.bind_seekable(audio, ($$value) => $.set(seekable, $$value));
	$.bind_seeking(audio, ($$value) => $.set(seeking, $$value));
	$.bind_ended(audio, ($$value) => $.set(ended, $$value));
	$.bind_ready_state(audio, ($$value) => $.set(readyState, $$value));
	$.bind_played(audio, ($$value) => $.set(played, $$value));
	$.append($$anchor, audio);
}
