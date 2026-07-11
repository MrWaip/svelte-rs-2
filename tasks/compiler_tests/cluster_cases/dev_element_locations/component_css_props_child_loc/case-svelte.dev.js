App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></span>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var node = $.child(span);
	{
		$.css_props(node, () => ({ "--color": "red" }));
		$.component(node.lastChild, () => $$props.Icon, ($$anchor, $$component) => {
			$$component($$anchor, {});
		});
		$.reset(node);
	}
	$.reset(span);
	$.append($$anchor, span);
	return $.pop($$exports);
}
