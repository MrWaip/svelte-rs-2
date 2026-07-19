App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>a</div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let arr = $.tag_proxy($.proxy({ test: () => {} }), "arr");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0) => ({ ...$0 }), [() => $.snapshot(arr, true)]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
