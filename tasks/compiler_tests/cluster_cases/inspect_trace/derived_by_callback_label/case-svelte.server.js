import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	let doubled = $.derived(() => {
		return count * 2;
	});
	$$renderer.push(`<button>${$.escape(doubled())}</button>`);
}
