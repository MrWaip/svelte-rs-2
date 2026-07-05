App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const noop = () => {};
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		if (noop) {
			const s = $.tag_proxy($.proxy({ x: 1 }), "s");
			noop(s);
		}
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
