import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = 10;
	let title2 = 12;
	title--;
	++title2;
	$$renderer.push(`<div${$.attr("attr", title++)}>_</div> ${$.escape(--title2)}`);
}
