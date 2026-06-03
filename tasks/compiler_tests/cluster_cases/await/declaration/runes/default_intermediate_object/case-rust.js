import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = $.proxy(Promise.resolve({}));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { p: { a } = {} } = $.get($$source);
			return { a };
		});
		var a = $.derived(() => $.get($$value).a);
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, $.get(a)));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
