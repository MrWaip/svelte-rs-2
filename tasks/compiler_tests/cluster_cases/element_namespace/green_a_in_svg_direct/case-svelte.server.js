import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><a><text>Hello</text></a></svg>`);
}
