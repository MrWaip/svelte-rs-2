App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img/>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let naturalWidth = $.tag($.state(0), "naturalWidth");
	let naturalHeight = $.tag($.state(0), "naturalHeight");
	var $$exports = { ...$.legacy_api() };
	var img = root();
	$.bind_property("naturalWidth", "load", img, function set($$value) {
		$.set(naturalWidth, $$value);
	});
	$.bind_property("naturalHeight", "load", img, function set($$value) {
		$.set(naturalHeight, $$value);
	});
	$.append($$anchor, img);
	return $.pop($$exports);
}
