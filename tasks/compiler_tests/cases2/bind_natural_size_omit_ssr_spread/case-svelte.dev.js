App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<img/>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let rest = {};
	let nw = $.tag($.state(0), "nw");
	let nh = $.tag($.state(0), "nh");
	var $$exports = { ...$.legacy_api() };
	var img = root();
	$.attribute_effect(img, () => ({
		alt: "",
		...rest
	}));
	$.replay_events(img);
	$.bind_property("naturalWidth", "load", img, function set($$value) {
		$.set(nw, $$value);
	});
	$.bind_property("naturalHeight", "load", img, function set($$value) {
		$.set(nh, $$value);
	});
	$.append($$anchor, img);
	return $.pop($$exports);
}
