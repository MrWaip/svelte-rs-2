App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<audio></audio> <video></video>`, 3), App[$.FILENAME], [[7, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let duration = $.tag($.state(0), "duration");
	let videoWidth = $.tag($.state(0), "videoWidth");
	let videoHeight = $.tag($.state(0), "videoHeight");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var audio = $.first_child(fragment);
	var video = $.sibling(audio, 2);
	$.bind_property("duration", "durationchange", audio, function set($$value) {
		$.set(duration, $$value);
	});
	$.bind_property("videoWidth", "resize", video, function set($$value) {
		$.set(videoWidth, $$value);
	});
	$.bind_property("videoHeight", "resize", video, function set($$value) {
		$.set(videoHeight, $$value);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
