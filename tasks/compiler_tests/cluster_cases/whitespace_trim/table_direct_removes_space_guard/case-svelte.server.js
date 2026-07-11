import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<table><tbody><tr><td>a</td></tr><tr><td>b</td></tr></tbody></table>`);
}
