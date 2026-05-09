import * as $ from "svelte/internal/client";
import Comp from "./Comp.svelte";
export default function App($$anchor) {
	let value = $.proxy({ name: "a" });
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const item = $.derived(() => value);
			Comp($$anchor, { get name() {
				return $.get(item).name;
			} });
		};
		$.if(node, ($$render) => {
			if (true) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
