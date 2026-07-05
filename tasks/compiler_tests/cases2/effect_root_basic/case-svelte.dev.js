App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	$.effect_root(() => {
		console.log("root effect:", count);
	});
	var $$exports = { ...$.legacy_api() };
	var p = root();
	p.textContent = "0";
	$.append($$anchor, p);
	return $.pop($$exports);
}
