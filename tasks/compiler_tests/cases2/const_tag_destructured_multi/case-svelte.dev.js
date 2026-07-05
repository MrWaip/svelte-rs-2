App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => $$props.items, $.index, ($$anchor, item) => {
		const computed_const = $.tag($.derived(() => {
			const { x, y } = $.get(item);
			return {
				x,
				y
			};
		}), "[@const]");
		$.get(computed_const);
		const computed_const_1 = $.tag($.derived(() => {
			const { a, b } = $.get(item);
			return {
				a,
				b
			};
		}), "[@const]");
		$.get(computed_const_1);
		var p = root();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${$.get(computed_const).x ?? ""} ${$.get(computed_const_1).a ?? ""}`));
		$.append($$anchor, p);
	}), "each", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
