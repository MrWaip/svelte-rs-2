App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<audio></audio>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let currentTime = $.tag($.state(0), "currentTime");
	let paused = $.tag($.state(true), "paused");
	let volume = $.tag($.state(1), "volume");
	let muted = $.tag($.state(false), "muted");
	let playbackRate = $.tag($.state(1), "playbackRate");
	var $$exports = { ...$.legacy_api() };
	var audio = root();
	$.bind_current_time(audio, function get() {
		return $.get(currentTime);
	}, function set($$value) {
		$.set(currentTime, $$value);
	});
	$.bind_paused(audio, function get() {
		return $.get(paused);
	}, function set($$value) {
		$.set(paused, $$value);
	});
	$.bind_volume(audio, function get() {
		return $.get(volume);
	}, function set($$value) {
		$.set(volume, $$value);
	});
	$.bind_muted(audio, function get() {
		return $.get(muted);
	}, function set($$value) {
		$.set(muted, $$value);
	});
	$.bind_playback_rate(audio, function get() {
		return $.get(playbackRate);
	}, function set($$value) {
		$.set(playbackRate, $$value);
	});
	$.append($$anchor, audio);
	return $.pop($$exports);
}
