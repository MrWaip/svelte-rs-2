import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<video></video>`, 2), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let w = $.prop($$props, "w", 12);
	var $$exports = { ...$.legacy_api() };
	var video = root();
	$.bind_property("videoWidth", "resize", video, function set($$value) {
		w($$value);
	});
	$.append($$anchor, video);
	return $.pop($$exports);
}
