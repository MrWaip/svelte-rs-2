import * as $ from "svelte/internal/server";
function shape($$renderer) {
	$$renderer.push(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`);
}
export default function App($$renderer) {
	shape($$renderer);
}
