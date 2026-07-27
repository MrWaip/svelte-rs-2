import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function f() {
		return 1;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.async(node, void 0, [async () => (await $.track_reactivity_loss(f()))()], ($$anchor, $0) => {
		$.css_props(node, () => ({ "--c": "1px" }));
		Child(node.lastChild, { get a() {
			return `y${$.get($0) ?? ""}`;
		} });
		$.reset(node);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
