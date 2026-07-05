import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[5, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const a = $.tag($.derived_safe_equal(() => "x"), "a");
			$.get(a);
			const b = $.tag($.derived_safe_equal(() => "y"), "b");
			$.get(b);
			var p = root();
			p.textContent = `${$.get(a) ?? ""}${$.get(b) ?? ""}`;
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 2, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
