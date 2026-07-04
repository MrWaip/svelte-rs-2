App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let size = 1;
	let arr = $.tag_proxy($.proxy([]), "arr");
	function joinClasses(a) {
		return a.join(" ");
	}
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.attribute_effect(div, ($0) => ({
		...{ id: "x" },
		class: `size_1 ${$0 ?? ""}`
	}), [() => joinClasses(arr)]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
