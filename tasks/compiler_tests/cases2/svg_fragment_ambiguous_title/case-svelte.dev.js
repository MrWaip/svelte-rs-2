App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<title>Chart</title>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_svg(`<svg><!></svg>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let shown = true;
	var $$exports = { ...$.legacy_api() };
	var svg = root_1();
	var node = $.child(svg);
	{
		var consequent = ($$anchor) => {
			var title = root();
			$.append($$anchor, title);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (shown) $$render(consequent);
		}), "if", App, 6, 1);
	}
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
