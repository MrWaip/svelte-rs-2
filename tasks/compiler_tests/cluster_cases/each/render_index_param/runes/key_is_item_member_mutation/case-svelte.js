import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{ name: "a" }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, (item) => item, ($$anchor, item, $$index) => {
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, item.name));
		$.delegated("click", button, () => item.name += "!");
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
