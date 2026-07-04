import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const Component = $.tag($.derived_safe_equal(() => Foo), "Component");
			$.get(Component);
			$.add_svelte_meta(() => $.get(Component)($$anchor, {}), "component", App, 7, 1, { componentTag: "Component" });
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (Foo) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
