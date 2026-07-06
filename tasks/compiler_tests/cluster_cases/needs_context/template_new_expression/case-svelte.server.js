import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let p = null;
		$$renderer.push(`<button>${$.escape(p)}</button>`);
	});
}
