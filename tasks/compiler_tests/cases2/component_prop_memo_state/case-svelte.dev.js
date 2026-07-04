App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.add_locations($.from_html(`<!> <button>bump</button>`, 1), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let i = $.tag($.state(0), "i");
	let index = 0;
	function bump() {
		$.update(i);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived(() => $.strict_equals($.get(i), index));
		$.add_svelte_meta(() => Comp(node, { get active() {
			return $.get($0);
		} }), "component", App, 9, 0, { componentTag: "Comp" });
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
