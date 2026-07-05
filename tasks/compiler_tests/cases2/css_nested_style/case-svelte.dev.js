App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<style>span {
      color: green;
    }</style>`), App[$.FILENAME], [[18, 2]]);
var root_1 = $.add_locations($.from_html(`<div class="svelte-19xqvng"><style>.nested {
      color: red;
    }</style> <p class="nested">inside div</p></div> <!>`, 1), App[$.FILENAME], [[
	7,
	0,
	[[8, 2], [14, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var style = root();
			$.append($$anchor, style);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 17, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
