import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#count = 0;
			count2 = 0;
			#doubled = $.derived(() => this.#count * 2);
			inc() {
				this.#count += 1;
				this.count2 += 1;
			}
			get value() {
				return this.#count;
			}
			get doubled() {
				return this.#doubled();
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>${$.escape(c.value)} ${$.escape(c.count2)} ${$.escape(c.doubled)}</button>`);
	});
}
