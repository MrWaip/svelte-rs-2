import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var count = 0;
	var name = "hello";
	count += 1;
	$$renderer.push(`<p>${$.escape(count)} hello</p>`);
}
