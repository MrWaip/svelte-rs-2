import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<my-element text="!"></my-element>`);
}
