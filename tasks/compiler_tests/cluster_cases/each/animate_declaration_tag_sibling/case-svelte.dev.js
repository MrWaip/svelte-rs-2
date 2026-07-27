App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function flip() {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 24, () => items, (n) => n, ($$anchor, n) => {
		const a = n;
		var div = root();
		$.animation(div, () => flip, () => a);
		$.append($$anchor, div);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
