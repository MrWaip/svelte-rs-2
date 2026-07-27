import * as $ from "svelte/internal/server";
function foo($$renderer, val) {
	$$renderer.push(`<!---->${$.escape(val)}`);
}
export default function App($$renderer) {}
