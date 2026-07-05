import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#n = 0;
			bump() {
				const self = this;
				self.#n += 1;
			}
			get n() {
				return this.#n;
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>${$.escape(c.n)}</button>`);
	});
}
