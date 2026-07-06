import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let selected = "a";
	$$renderer.select({ value: selected }, ($$renderer) => {});
}
