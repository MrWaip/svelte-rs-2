import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let num = $.state(0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => $$props.object, null, ($$anchor, $$source) => {
		var $$value = $.derived(() => {
			var { [`k${$.update(num)}`]: v } = $.get($$source);
			return { v };
		});
		var v = $.derived(() => $.get($$value).v);
		var button = root();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(v) ?? ""} ${$.get(num) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
