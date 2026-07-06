import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Store {
			#data = { n: 0 };
			get data() {
				return this.#data;
			}
			inc() {
				this.#data.n++;
			}
		}
		const store = new Store();
		$$renderer.push(`<button>${$.escape(store.data.n)}</button>`);
	});
}
