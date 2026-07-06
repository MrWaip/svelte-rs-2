import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let fns = [() => {}];
		let n = 0;
		$$renderer.push(`<button>${$.escape(n)}</button>`);
	});
}
