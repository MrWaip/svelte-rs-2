import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let buffered = void 0;
	let seekable = void 0;
	let seeking = false;
	let ended = false;
	let readyState = 0;
	let played = void 0;
	$$renderer.push(`<audio></audio>`);
}
