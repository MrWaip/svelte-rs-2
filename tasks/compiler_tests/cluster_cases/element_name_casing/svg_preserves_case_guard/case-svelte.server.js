import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<svg><clipPath id="c"></clipPath><linearGradient id="g"></linearGradient></svg>`);
}
