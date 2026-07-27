import * as $ from "svelte/internal/server";
function generic($$renderer, val) {
	$$renderer.push(`<!---->${$.escape(val)}`);
}
export default function App($$renderer) {}
