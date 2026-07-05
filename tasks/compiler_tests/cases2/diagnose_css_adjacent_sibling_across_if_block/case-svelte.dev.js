App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div class="b svelte-13830z5"></div>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_html(`<div class="c svelte-13830z5"></div>`), App[$.FILENAME], [[10, 2]]);
var root_2 = $.add_locations($.from_html(`<div class="a svelte-13830z5"></div> <!> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.sibling($.first_child(fragment), 2);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($$props.x) $$render(consequent);
		}), "if", App, 6, 0);
	}
	var node_1 = $.sibling(node, 2);
	{
		var consequent_1 = ($$anchor) => {
			var div_1 = root_1();
			$.append($$anchor, div_1);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if ($$props.y) $$render(consequent_1);
		}), "if", App, 9, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
