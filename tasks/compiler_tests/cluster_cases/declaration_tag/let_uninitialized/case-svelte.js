import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, item) => {
		let a, b;
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${a ?? ""} ${b ?? ""}`));
		$.delegated("click", button, () => {
			a = item;
			b = item;
		});
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
