import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let currentTime = 0;
	let paused = true;
	let volume = 1;
	let muted = false;
	let playbackRate = 1;
	$$renderer.push(`<audio></audio>`);
}
