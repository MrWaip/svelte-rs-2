import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>ok</p> <!>`, 1), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = [1, 2];
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => items, $.index, ($$anchor, item) => {
		var fragment_1 = root();
		var node_1 = $.sibling($.first_child(fragment_1), 2);
		$.add_svelte_meta(() => App(node_1, { get value() {
			return $.get(item);
		} }), "component", App, 7, 1, { componentTag: "svelte:self" });
		$.append($$anchor, fragment_1);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
