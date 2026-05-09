import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root_1 = $.from_html(`<span>hi</span>`);
export default function App($$anchor) {
	{
		const extra_element = ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		};
		Child($$anchor, {
			extra_element,
			$$slots: { extra_element: true }
		});
	}
}
