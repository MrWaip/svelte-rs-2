import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let content = "<mi>x</mi>";
	$$renderer.push(`<math><mn>1</mn> ${$.html(content)}</math>`);
}
