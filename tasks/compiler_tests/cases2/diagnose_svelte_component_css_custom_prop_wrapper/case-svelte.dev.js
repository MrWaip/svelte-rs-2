App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Icon from "./Icon.svelte";
var root = $.add_locations($.from_html(`<span class="wrap svelte-1fxeua7"><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></span>`), App[$.FILENAME], [[
	7,
	0,
	[[8, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let current = Icon;
	var $$exports = { ...$.legacy_api() };
	var span = root();
	var node = $.child(span);
	{
		$.css_props(node, () => ({ "--my-color": `var(--${$$props.color ?? ""})` }));
		$.component(node.lastChild, () => current, ($$anchor, $$component) => {
			$$component($$anchor, {});
		});
		$.reset(node);
	}
	$.reset(span);
	$.append($$anchor, span);
	return $.pop($$exports);
}
