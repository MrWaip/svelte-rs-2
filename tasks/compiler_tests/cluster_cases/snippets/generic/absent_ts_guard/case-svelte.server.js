import * as $ from "svelte/internal/server";
function plain($$renderer, val) {
	$$renderer.push(`<!---->${$.escape(val)}`);
}
export default function App($$renderer) {}
