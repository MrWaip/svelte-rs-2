import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let lib = $.prop($$props, "lib", 8, undefined);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const L = $.derived_safe_equal(lib);
			$.get(L).Button($$anchor, {});
		};
		$.if(node, ($$render) => {
			if (lib()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
