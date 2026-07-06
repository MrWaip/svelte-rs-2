import * as $ from "svelte/internal/server";
import { fade, fly } from "svelte/transition";
export default function App($$renderer) {
	const handler = () => {};
	$$renderer.push(`<div><div></div></div>`);
}
