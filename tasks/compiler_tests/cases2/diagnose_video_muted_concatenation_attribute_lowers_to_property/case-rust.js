import * as $ from "svelte/internal/client";
var root = $.from_html(`<video src="x.mp4" autoplay=""></video>`, 2);
export default function App($$anchor, $$props) {
	var video = root();
	$.template_effect(() => video.muted = `${$$props.prefix ?? ""}${$$props.suffix ?? ""}`);
	$.append($$anchor, video);
}
