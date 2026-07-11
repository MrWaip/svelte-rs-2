App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--color": "red" }));
		Component(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
