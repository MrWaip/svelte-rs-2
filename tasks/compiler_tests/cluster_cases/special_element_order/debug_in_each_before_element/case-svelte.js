import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([1, 2]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item) => {
		var button = root();
		$.template_effect(() => {
			console.log({ item: $.snapshot($.get(item)) });
			debugger;
		});
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.delegated("click", button, () => items.push(0));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
