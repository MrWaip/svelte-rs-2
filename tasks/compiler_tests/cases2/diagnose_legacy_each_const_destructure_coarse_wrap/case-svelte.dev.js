import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
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
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, rows, (row) => row.key, ($$anchor, row) => {
		const computed_const = $.tag($.derived_safe_equal(() => {
			const { x, y } = ($.get(row), $.untrack(() => lookup[$.get(row).key]));
			return {
				x,
				y
			};
		}), "[@const]");
		$.get(computed_const);
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(computed_const).x ?? ""}:${$.get(computed_const).y ?? ""}`));
		$.append($$anchor, p);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
