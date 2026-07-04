App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<video src="x.mp4" autoplay=""></video>`, 2), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var video = root();
	$.template_effect(() => video.muted = `${$$props.prefix ?? ""}${$$props.suffix ?? ""}`);
	$.append($$anchor, video);
	return $.pop($$exports);
}
