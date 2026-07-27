import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<button>inc</button> <svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper>`, 1), App[$.FILENAME], [[11, 0], [14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, void 0, [async () => (await $.track_reactivity_loss(delay($.get(x))))()], ($$anchor, $0) => {
		$.css_props(node, () => ({ "--color": $.get($0) }));
		Child(node.lastChild, {});
		$.reset(node);
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
