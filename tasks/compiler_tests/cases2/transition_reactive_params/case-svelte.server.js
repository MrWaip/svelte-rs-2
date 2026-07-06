import * as $ from "svelte/internal/server";
import { fly } from "svelte/transition";
export default function App($$renderer) {
	let y = 200;
	$$renderer.push(`<div>hello</div>`);
}
