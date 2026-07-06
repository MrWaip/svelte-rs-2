import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	/** @type {Function | undefined} */
	let show = void 0;
	/** Toggles the optional callback when clicked. */
	function toggle() {
		if (show) show();
	}
	$$renderer.push(`<button>toggle</button>`);
}
