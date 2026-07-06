import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = 10;
	$$renderer.push(`<!---->${$.escape(title = 30)}`);
}
