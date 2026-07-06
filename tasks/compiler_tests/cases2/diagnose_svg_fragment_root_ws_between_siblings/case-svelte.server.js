import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><path d="M1"></path></svg><g><path d="M2"></path></g>`);
}
