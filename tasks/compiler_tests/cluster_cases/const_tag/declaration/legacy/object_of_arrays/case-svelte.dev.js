import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let pairs = {
		p: [1, 2],
		q: [3, 4]
	};
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.tag($.derived_safe_equal(() => {
				const { p: [a, b], q: [c, d] } = pairs;
				return {
					a,
					b,
					c,
					d
				};
			}), "[@const]");
			$.get(computed_const);
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).a ?? ""}${$.get(computed_const).b ?? ""}${$.get(computed_const).c ?? ""}${$.get(computed_const).d ?? ""}`));
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
