import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let content = "<p>safe</p>";
	$$renderer.push(`${$.html(content)}`);
}
