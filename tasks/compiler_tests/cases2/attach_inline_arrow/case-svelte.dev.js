App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>content</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let message = "hello";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attach(div, () => (el) => {
		el.textContent = message;
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
