import * as $ from "svelte/internal/client";
var root = $.from_html(`<audio></audio> <video></video>`, 3);
export default function App($$anchor) {
	let duration = $.state(0);
	let videoWidth = $.state(0);
	let videoHeight = $.state(0);
	var fragment = root();
	var audio = $.first_child(fragment);
	var video = $.sibling(audio, 2);
	$.bind_property("duration", "durationchange", audio, ($$value) => $.set(duration, $$value));
	$.bind_property("videoWidth", "resize", video, ($$value) => $.set(videoWidth, $$value));
	$.bind_property("videoHeight", "resize", video, ($$value) => $.set(videoHeight, $$value));
	$.append($$anchor, fragment);
}
