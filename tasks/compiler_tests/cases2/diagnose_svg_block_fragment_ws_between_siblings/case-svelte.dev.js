App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1), App[$.FILENAME], [[
	2,
	1,
	[[2, 4]]
], [
	3,
	1,
	[[3, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = root();
			$.next();
			$.append($$anchor, fragment_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (cond) $$render(consequent);
		}), "if", App, 1, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
