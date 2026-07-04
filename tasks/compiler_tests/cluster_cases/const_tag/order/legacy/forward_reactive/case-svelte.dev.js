import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let n = $.tag($.mutable_source(1), "n");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const bar = $.tag($.derived_safe_equal(() => $.get(n)), "bar");
			$.get(bar);
			const foo = $.tag($.derived_safe_equal(() => $.get(bar)), "foo");
			$.get(foo);
			var button = root();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, $.get(foo)));
			$.event("click", button, function click() {
				return $.set(n, $.get(n) + 1);
			});
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(n)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
