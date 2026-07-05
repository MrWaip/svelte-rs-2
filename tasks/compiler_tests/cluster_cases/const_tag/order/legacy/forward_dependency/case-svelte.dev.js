import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1></h1>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const bar = $.tag($.derived_safe_equal(() => "world"), "bar");
			$.get(bar);
			const foo = $.tag($.derived_safe_equal(() => $.get(bar)), "foo");
			$.get(foo);
			const yoo = $.tag($.derived_safe_equal(() => $.get(foo)), "yoo");
			$.get(yoo);
			var h1 = root();
			h1.textContent = `Hello ${$.get(bar) ?? ""}${$.get(yoo) ?? ""}!`;
			$.append($$anchor, h1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 2, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
