App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<g><path d="M1"></path></g>`), App[$.FILENAME], [[
	8,
	1,
	[[8, 4]]
]]);
var root_1 = $.add_locations($.from_svg(`<!><!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let cond = true;
	let raw = "<circle r={5}/>";
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.html(node, () => raw, void 0, true);
	var node_1 = $.sibling(node);
	{
		var consequent = ($$anchor) => {
			var g = root();
			$.append($$anchor, g);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (cond) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
