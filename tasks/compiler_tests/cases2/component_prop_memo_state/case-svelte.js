import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
var root = $.from_html(`<!> <button>bump</button>`, 1);
export default function App($$anchor) {
	let i = $.state(0);
	let index = 0;
	function bump() {
		$.update(i);
	}
	var fragment = root();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived(() => $.get(i) === index);
		Comp(node, { get active() {
			return $.get($0);
		} });
	}
	var button = $.sibling(node, 2);
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
