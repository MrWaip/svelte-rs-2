App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { SvelteSet } from "svelte/reactivity";
var root = $.add_locations($.from_html(`<button>add</button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const set = new SvelteSet();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		const s = $.tag_proxy($.proxy({ x: 1 }), "s");
		set.add(s);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
