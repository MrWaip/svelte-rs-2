import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div></div> <my-thing></my-thing>`);
}
