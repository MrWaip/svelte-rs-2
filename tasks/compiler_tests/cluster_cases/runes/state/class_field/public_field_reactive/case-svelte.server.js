import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Counter {
			count = 0;
			#double = $.derived(() => this.count * 2);
			get double() {
				return this.#double();
			}
			set double($$value) {
				return this.#double($$value);
			}
			inc() {
				this.count += 1;
			}
			get viaAlias() {
				const self = this;
				return self.count;
			}
		}
		const c = new Counter();
		$$renderer.push(`<button>${$.escape(c.count)} ${$.escape(c.double)} ${$.escape(c.viaAlias)}</button>`);
	});
}
