import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<a> </a>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, $.index, ($$anchor, item) => {
		const computed_const = $.derived_safe_equal(() => {
			const { button } = $.get(item);
			return { button };
		});
		var a = root_1();
		var text = $.child(a, true);
		$.reset(a);
		$.template_effect(() => {
			$.set_attribute(a, "href", ($.deep_read_state($.get(computed_const).button), $.untrack(() => $.get(computed_const).button.href)));
			$.set_text(text, ($.deep_read_state($.get(computed_const).button), $.untrack(() => $.get(computed_const).button.text)));
		});
		$.append($$anchor, a);
	});
	$.append($$anchor, fragment);
}
