App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let animated = $.tag($.state(false), "animated");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.transition(2, div, () => fade);
	$.event("introstart", div, function introstart() {
		return $.set(animated, true);
	});
	$.event("outroend", div, function outroend() {
		return $.set(animated, false);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
