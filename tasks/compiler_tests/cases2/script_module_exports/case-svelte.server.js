import * as $ from "svelte/internal/server";
export const API_URL = "/api";
export function formatDate(d) {
	return d.toISOString();
}
export default function App($$renderer) {
	let value = "hello";
	$$renderer.push(`<p>hello</p>`);
}
