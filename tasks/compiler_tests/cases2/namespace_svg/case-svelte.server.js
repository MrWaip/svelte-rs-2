import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<rect width="100" height="100"></rect>`);
}
