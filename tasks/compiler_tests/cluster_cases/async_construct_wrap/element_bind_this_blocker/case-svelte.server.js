import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	var ref;
	var $$promises = $$renderer.run([() => Promise.resolve(), () => ref = null]);
	$$renderer.push(`<div></div>`);
}
