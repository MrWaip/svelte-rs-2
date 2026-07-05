import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let count = 0;
	const label = $.derived(() => {
		if (!count) return "a";
		return `a ${count}`;
	});
	$$renderer.push(`<button>${$.escape(label())}</button>`);
}
