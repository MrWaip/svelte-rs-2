import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	var x;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => x = 2]);
	$$renderer.async([$$promises[1]], ($$renderer) => {
		$$renderer.push(`<div${$.attr("title", x)} class="a2b"></div>`);
	});
	$$renderer.push(` `);
	$$renderer.async_block([$$promises[1]], ($$renderer) => {
		Child($$renderer, {
			a: () => x,
			b: { k: x }
		});
	});
}
