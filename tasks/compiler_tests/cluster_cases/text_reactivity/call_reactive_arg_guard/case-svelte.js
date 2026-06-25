import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p> <button>+</button>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	function fn(x) {
		return x;
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(($0) => $.set_text(text, `v ${$0 ?? ""}`), [() => fn($.get(count))]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
