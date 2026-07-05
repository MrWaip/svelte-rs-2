import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function inner($$renderer, mf) {
		$$renderer.push(`<div><div><div><div><span>a</span></div></div></div> <div><!---->`);
		{
			mf($$renderer);
			$$renderer.push(`<!---->`);
		}
		$$renderer.push(`<!----></div></div>`);
	}
}
