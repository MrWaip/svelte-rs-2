import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let pairs = {
		"a-b": 1,
		"c d": 2
	};
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const computed_const = $.tag($.derived_safe_equal(() => {
				const { "a-b": ab, "c d": cd } = pairs;
				return {
					ab,
					cd
				};
			}), "[@const]");
			$.get(computed_const);
			var button = root();
			var text = $.child(button);
			$.reset(button);
			$.template_effect(() => $.set_text(text, `${$.get(computed_const).ab ?? ""}${$.get(computed_const).cd ?? ""}`));
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (pairs) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
