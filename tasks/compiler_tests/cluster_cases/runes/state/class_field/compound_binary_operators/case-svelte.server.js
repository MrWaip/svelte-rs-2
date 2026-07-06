import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#n = 0;
			bump() {
				this.#n += 1;
				this.#n <<= 2;
				this.#n >>>= 1;
				this.#n &= 6;
			}
			get n() {
				return this.#n;
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>${$.escape(c.n)}</button>`);
	});
}
