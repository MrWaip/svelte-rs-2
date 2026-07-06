import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let title = 10;
	let other = 20;
	$$renderer.push(`<!---->${$.escape(title += 5)}
${$.escape(title &&= other)}`);
}
