import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	Button($$renderer, {});
	$$renderer.push(`<!----></div>`);
}
