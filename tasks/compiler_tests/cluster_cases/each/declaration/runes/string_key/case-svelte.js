import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let items = $.proxy([{
		"a-b": 1,
		"c d": 2
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item) => {
		let ab = () => $.get($$item)["a-b"];
		let cd = () => $.get($$item)["c d"];
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${ab() ?? ""}${cd() ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
