import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Store {
			items = [];
		}
		let s = new Store();
		$$renderer.push(`<p>${$.escape(s.items)}</p>`);
	});
}
