import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { message } = $$props;
	function report() {
		console.error("Error: " + message);
	}
	$$renderer.push(`<button>go</button>`);
}
