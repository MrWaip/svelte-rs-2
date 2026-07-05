import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>a</p>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<p>b</p> <!>`, 1), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		var alternate = ($$anchor) => {
			var fragment_1 = root_1();
			var node_1 = $.sibling($.first_child(fragment_1), 2);
			$.add_svelte_meta(() => App(node_1, {}), "component", App, 9, 1, { componentTag: "svelte:self" });
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (count > 0) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
