import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let a = 0;
		const c = $.derived(() => class {}(a));
		$$renderer.push(`<button>${$.escape(typeof c())}</button>`);
	});
}
