import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const k = "z";
	let p = $.proxy(Promise.resolve({ z: 1 }));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { [k]: v } = $.get($$source);
			return { v };
		});
		var v = $.derived(() => $.get($$value).v);
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(v)));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
