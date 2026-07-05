App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>fallback html</div>`), App[$.FILENAME], [[9, 2]]);
var root_1 = $.add_locations($.from_svg(`<foreignObject><!></foreignObject>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let shown = true;
	var $$exports = { ...$.legacy_api() };
	var foreignObject = root_1();
	var node = $.child(foreignObject);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (shown) $$render(consequent);
		}), "if", App, 8, 1);
	}
	$.reset(foreignObject);
	$.append($$anchor, foreignObject);
	return $.pop($$exports);
}
