import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let items = $.proxy([() => {}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, handler) => {
		var button = root_1();
		$.delegated("click", button, function(...$$args) {
			$.get(handler)?.apply(this, $$args);
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
