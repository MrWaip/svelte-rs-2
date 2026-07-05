import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Store {
			data = null;
			update(val) {
				this.data = val;
			}
		}
		let s = new Store();
		$$renderer.push(`<p>${$.escape(s.data)}</p>`);
	});
}
