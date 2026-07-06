import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "rect";
	let ns = "http://www.w3.org/2000/svg";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attr("xmlns", ns)} width="100" height="100"`);
	});
}
