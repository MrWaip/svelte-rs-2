import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{
		id: 1,
		name: "a"
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, (item) => item.id, ($$anchor, item, $$index) => {
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(item).name));
		$.delegated("click", button, () => $.get(item).name += "!");
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
