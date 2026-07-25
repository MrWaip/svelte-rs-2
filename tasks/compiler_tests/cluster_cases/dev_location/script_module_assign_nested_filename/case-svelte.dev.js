App[$.FILENAME] = "src/lib/Widget.svelte";
import * as $ from "svelte/internal/client";
const cache = {};
export function fill(items) {
	items.forEach((item) => $.assign(cache, item.id, "=", item, "src/​lib/​Widget.svelte:4:26"));
}
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return fill([]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
