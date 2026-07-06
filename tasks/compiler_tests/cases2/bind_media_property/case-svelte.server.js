import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let duration = 0;
	let videoWidth = 0;
	let videoHeight = 0;
	$$renderer.push(`<audio></audio> <video></video>`);
}
