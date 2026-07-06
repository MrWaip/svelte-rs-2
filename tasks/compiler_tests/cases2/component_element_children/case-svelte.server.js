import * as $ from "svelte/internal/server";
import Card from "./Card.svelte";
export default function App($$renderer) {
	Card($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<p>Hello world</p>`);
		},
		$$slots: { default: true }
	});
}
