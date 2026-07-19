import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let maybeNull = null;
	let maybeUndefined = undefined;
	$$renderer.push(`<p></p> <p></p>`);
}
