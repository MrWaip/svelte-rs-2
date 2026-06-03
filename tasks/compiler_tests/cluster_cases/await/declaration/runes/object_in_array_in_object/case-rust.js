import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve({ outer: [{ inner: 1 }] }));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { outer: [{ inner }] } = $.get($$source);
			return { inner };
		});
		var inner = $.derived(() => $.get($$value).inner);
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(inner)));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
