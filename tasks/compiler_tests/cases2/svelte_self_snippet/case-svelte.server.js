import * as $ from "svelte/internal/server";
function recurse($$renderer) {
	App($$renderer, {});
	$$renderer.push(`<!---->`);
}
export default function App($$renderer) {
	recurse($$renderer);
}
