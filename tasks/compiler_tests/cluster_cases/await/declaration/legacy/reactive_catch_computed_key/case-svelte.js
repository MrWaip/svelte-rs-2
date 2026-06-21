import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.reject({ k1: 1 });
	let num = $.mutable_source(0);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, x) => {
		var text = $.text("ok");
		$.append($$anchor, text);
	}, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { [`k${$.update(num)}`]: v } = $.get($$source);
			return { v };
		});
		var v = $.derived_safe_equal(() => $.get($$value).v);
		var button = root_2();
		var text_1 = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text_1, `${$.get(v) ?? ""} ${$.get(num) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
