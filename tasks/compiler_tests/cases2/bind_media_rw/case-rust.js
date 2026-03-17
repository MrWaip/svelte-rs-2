import * as $ from "svelte/internal/client";
var root = $.from_html(`<audio></audio>`);
export default function App($$anchor) {
	let currentTime = $.state(0);
	let paused = $.state(true);
	let volume = $.state(1);
	let muted = $.state(false);
	let playbackRate = $.state(1);
	var audio = root();
	$.bind_current_time(audio, () => $.get(currentTime), ($$value) => $.set(currentTime, $$value));
	$.bind_paused(audio, () => $.get(paused), ($$value) => $.set(paused, $$value));
	$.bind_volume(audio, () => $.get(volume), ($$value) => $.set(volume, $$value));
	$.bind_muted(audio, () => $.get(muted), ($$value) => $.set(muted, $$value));
	$.bind_playback_rate(audio, () => $.get(playbackRate), ($$value) => $.set(playbackRate, $$value));
	$.append($$anchor, audio);
}
