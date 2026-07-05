import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[14, 4]]);
var root_1 = $.add_locations($.from_html(` <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	let item = $.prop($$props, "item", 8);
	let target;
	function handle(e) {
		[target] = e;
	}
	var $$exports = { ...$.legacy_api() };
	$.next();
	var fragment = root_1();
	var text = $.first_child(fragment);
	text.nodeValue = `${handle} `;
	var node = $.sibling(text);
	$.add_svelte_meta(() => $.each(node, 1, items, (item) => item.id, ($$anchor, item, $$index, $$array_1) => {
		var div = root();
		var text_1 = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text_1, ($.get(item), $.untrack(() => $.get(item).id))));
		$.append($$anchor, div);
	}), "each", App, 13, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
