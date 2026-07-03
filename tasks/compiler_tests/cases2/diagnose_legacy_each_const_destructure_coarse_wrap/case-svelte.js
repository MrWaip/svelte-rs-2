import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	let rows = $.prop($$props, "rows", 8);
	const lookup = {
		a: {
			x: 1,
			y: 2
		},
		b: {
			x: 3,
			y: 4
		}
	};
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		const computed_const = $.derived_safe_equal(() => {
			const { x, y } = ($.get(row), $.untrack(() => lookup[$.get(row).key]));
			return {
				x,
				y
			};
		});
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(computed_const).x ?? ""}:${$.get(computed_const).y ?? ""}`));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
