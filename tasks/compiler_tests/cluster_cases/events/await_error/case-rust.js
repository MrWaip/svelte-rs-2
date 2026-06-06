import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let promise = Promise.reject();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, null, void 0, ($$anchor, handler) => {
		var button = root_1();
		$.delegated("click", button, function(...$$args) {
			$.get(handler)?.apply(this, $$args);
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
