App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
import B from "./B.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = 0;
	let Comp = $.tag($.derived(() => $.strict_equals(value % 2, 0) ? A : B), "Comp");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--prop": "red" }));
		$.component(node.lastChild, () => $.get(Comp), ($$anchor, Comp_1) => {
			Comp_1($$anchor, {});
		});
		$.reset(node);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
