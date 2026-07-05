import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			#doubled;
			get doubled() {
				return this.#doubled();
			}
			set doubled(value) {
				this.#doubled(value);
			}
			#count;
			constructor(initial) {
				this.#count = initial;
				this.#doubled = $.derived(() => this.#count * 2);
			}
			increment = () => {
				this.#count++;
			};
		}
		const counter = new Counter(10);
		$$renderer.push(`<button>${$.escape(counter.doubled)}</button>`);
	});
}
