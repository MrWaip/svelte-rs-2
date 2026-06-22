import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<video></video>`, 2);
export default function App($$anchor, $$props) {
	let d = $.prop($$props, "d", 12);
	var video = root();
	$.bind_property("duration", "durationchange", video, d);
	$.append($$anchor, video);
}
