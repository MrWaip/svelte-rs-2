import * as $ from "svelte/internal/server";
function row($$renderer, item) {
	const label = item.name;
	$$renderer.push(`<p>${$.escape(label)}</p>`);
}
export default function App($$renderer) {
	row($$renderer, { name: "x" });
}
