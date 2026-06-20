import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<video></video>`, 2);
export default function App($$anchor, $$props) {
	let w = $.prop($$props, "w", 12);
	var video = root();
	$.bind_property("videoWidth", "resize", video, w);
	$.append($$anchor, video);
}
