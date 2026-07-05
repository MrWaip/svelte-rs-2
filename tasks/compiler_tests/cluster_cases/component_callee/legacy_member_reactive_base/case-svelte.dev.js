import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let lib = $.prop($$props, "lib", 8, undefined);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const L = $.tag($.derived_safe_equal(lib), "L");
			$.get(L);
			$.add_svelte_meta(() => $.get(L).Button($$anchor, {}), "component", App, 7, 1, { componentTag: "L.Button" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (lib()) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
