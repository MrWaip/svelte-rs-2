import * as $ from "svelte/internal/server";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let animated = false;
	$$renderer.push(`<div></div>`);
}
