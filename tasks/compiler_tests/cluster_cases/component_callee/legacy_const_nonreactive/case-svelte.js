import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const Component = $.derived_safe_equal(() => Foo);
			$.get(Component)($$anchor, {});
		};
		$.if(node, ($$render) => {
			if (Foo) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
