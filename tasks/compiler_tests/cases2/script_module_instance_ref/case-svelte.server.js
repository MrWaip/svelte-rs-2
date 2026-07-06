import * as $ from "svelte/internal/server";
const BASE = "https://example.com";
export default function App($$renderer) {
	let path = "/home";
	let url = $.derived(() => BASE + path);
	$$renderer.push(`<a${$.attr("href", url())}>Link</a>`);
}
