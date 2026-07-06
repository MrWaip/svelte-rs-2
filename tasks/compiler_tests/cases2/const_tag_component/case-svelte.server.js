import * as $ from "svelte/internal/server";
import Widget from "./Widget.svelte";
export default function App($$renderer) {
	let value = 5;
	Widget($$renderer, {
		children: ($$renderer) => {
			const doubled = value * 2;
			$$renderer.push(`<p>10</p>`);
		},
		$$slots: { default: true }
	});
}
