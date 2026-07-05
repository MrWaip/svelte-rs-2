import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<hr/> <!>`, 1), App[$.FILENAME], [[4, 2]]);
var root_1 = $.add_locations($.from_html(`<div><!> <!></div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$slots = $.sanitize_slots($$props);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.slot(node, $$props, "title", {}, null);
	var node_1 = $.sibling(node, 2);
	{
		var consequent = ($$anchor) => {
			var fragment = root();
			var node_2 = $.sibling($.first_child(fragment), 2);
			$.slot(node_2, $$props, "description", {}, null);
			$.append($$anchor, fragment);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if ($.untrack(() => $$slots.description)) $$render(consequent);
		}), "if", App, 3, 1);
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
