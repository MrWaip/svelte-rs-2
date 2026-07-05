import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<span>hi</span>`);
export default function App($$anchor) {
	{
		const extra_element = ($$anchor) => {
			var span = root();
			$.append($$anchor, span);
		};
		Child($$anchor, {
			extra_element,
			$$slots: { extra_element: true }
		});
	}
}
