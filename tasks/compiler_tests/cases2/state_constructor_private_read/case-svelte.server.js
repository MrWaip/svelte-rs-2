import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Timer {
			#elapsed = 0;
			tick() {
				this.#elapsed += 1;
			}
			get display() {
				return this.#elapsed;
			}
		}
		let t = new Timer();
		$$renderer.push(`<p>${$.escape(t.display)}</p>`);
	});
}
