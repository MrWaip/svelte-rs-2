import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, item, i) => {
		var div = root();
		div.textContent = i + 1;
		$.template_effect(() => $.set_class(div, 1, `${($.get(item), $.untrack(() => $.get(item).foo ? "foo" : "")) ?? ""} ${($.get(item), $.untrack(() => $.get(item).bar ? "bar" : "")) ?? ""}`));
		$.append($$anchor, div);
	}), "each", App, 4, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
