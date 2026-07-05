import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
export default function App($$renderer) {
	let count = 0;
	Button($$renderer, {
		theme: "primary",
		onclick: () => count++,
		children: ($$renderer) => {
			$$renderer.push(`<!---->Clicked ${$.escape(count)} times`);
		},
		$$slots: { default: true }
	});
}
