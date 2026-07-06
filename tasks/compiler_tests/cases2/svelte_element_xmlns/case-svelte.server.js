import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "rect";
	$.element($$renderer, tag, () => {
		$$renderer.push(` xmlns="http://www.w3.org/2000/svg" width="100" height="100"`);
	});
}
