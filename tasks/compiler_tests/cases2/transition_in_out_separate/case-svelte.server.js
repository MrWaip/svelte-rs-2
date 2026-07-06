import * as $ from "svelte/internal/server";
import { fly, fade } from "svelte/transition";
export default function App($$renderer) {
	$$renderer.push(`<div>hello</div>`);
}
