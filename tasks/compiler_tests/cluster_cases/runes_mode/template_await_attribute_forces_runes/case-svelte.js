import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>hi</p> <button>inc</button>`, 1);
export default function App($$anchor) {
	let count = 0;
	async function compute(v) {
		return v * 2;
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var button = $.sibling(p, 2);
	$.template_effect(($0) => $.set_attribute(p, "title", $0), void 0, [() => compute(count)]);
	$.delegated("click", button, () => count++);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
