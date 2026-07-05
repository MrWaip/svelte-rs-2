import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let promise = Promise.resolve(() => {});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, ($$anchor, handler) => {
		var button = root();
		$.delegated("click", button, function(...$$args) {
			$.get(handler)?.apply(this, $$args);
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
