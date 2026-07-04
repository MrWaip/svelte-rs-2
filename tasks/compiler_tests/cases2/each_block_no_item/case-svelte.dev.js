App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>item</div>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 16, () => items, $.index, ($$anchor, $$item) => {
		var div = root();
		$.append($$anchor, div);
	}), "each", App, 1, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
