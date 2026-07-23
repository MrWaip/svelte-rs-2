import * as $ from "svelte/internal/server";
function complex_generic($$renderer, val) {
	$$renderer.push(`<!---->${$.escape(val)}`);
}
export default function App($$renderer) {}
