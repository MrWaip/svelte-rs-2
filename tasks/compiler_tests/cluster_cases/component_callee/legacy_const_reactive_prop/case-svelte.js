import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	let component = $.prop($$props, "component", 8, undefined);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const Component = $.derived_safe_equal(component);
			$.get(Component)($$anchor, {});
		};
		$.if(node, ($$render) => {
			if (component()) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
