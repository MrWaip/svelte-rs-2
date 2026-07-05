import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button> <!>`, 1);
export default function App($$anchor, $$props) {
	let count = $.state(0);
	function label(n) {
		return n + 1;
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		let $0 = $.derived(() => label($.get(count)));
		$.snippet(node, () => $$props.children, () => $.get($0));
	}
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
