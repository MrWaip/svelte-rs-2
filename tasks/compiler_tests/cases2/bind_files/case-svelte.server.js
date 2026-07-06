import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let files = void 0;
	$$renderer.push(`<input type="file"/>`);
}
