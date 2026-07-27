import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<a href="/foo">home</a>`);
}
