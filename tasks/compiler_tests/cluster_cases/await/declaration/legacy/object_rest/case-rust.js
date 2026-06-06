import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let p = Promise.resolve({
		a: 1,
		b: 2,
		c: 3
	});
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => p, null, ($$anchor, $$source) => {
		var $$value = $.derived_safe_equal(() => {
			var { a, ...rest } = $.get($$source);
			return {
				a,
				rest
			};
		});
		var a = $.derived_safe_equal(() => $.get($$value).a);
		var rest = $.derived_safe_equal(() => $.get($$value).rest);
		var button = root_1();
		var text = $.child(button);
		$.reset(button);
		$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${($.deep_read_state($.get(rest)), $.untrack(() => $.get(rest).b)) ?? ""}`));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
