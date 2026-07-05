import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const c = $.tag($.derived_safe_equal(() => "x"), "c");
			$.get(c);
			const b = $.tag($.derived_safe_equal(() => $.get(c)), "b");
			$.get(b);
			const a = $.tag($.derived_safe_equal(() => $.get(b)), "a");
			$.get(a);
			var p = root();
			p.textContent = $.get(a);
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 2, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
