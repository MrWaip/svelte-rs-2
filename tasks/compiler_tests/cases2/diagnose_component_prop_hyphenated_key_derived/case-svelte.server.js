import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer) {
	let count = 0;
	$$renderer.push(`<button>${$.escape(count)}</button> `);
	Child($$renderer, {
		"aria-disabled": !count,
		children: ($$renderer) => {
			$$renderer.push(`<!---->hi`);
		},
		$$slots: { default: true }
	});
	$$renderer.push(`<!---->`);
}
