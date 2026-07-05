App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<canvas></canvas>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let el = $.tag($.state(void 0), "el");
	var $$exports = { ...$.legacy_api() };
	var canvas = root();
	$.bind_this(canvas, ($$value) => $.set(el, $$value), () => $.get(el));
	$.append($$anchor, canvas);
	return $.pop($$exports);
}
