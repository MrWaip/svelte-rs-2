import * as $ from "svelte/internal/client";
var root = $.from_html(`<video></video>`, 2);
export default function App($$anchor) {
	var video = root();
	video.muted = true;
	$.append($$anchor, video);
}
