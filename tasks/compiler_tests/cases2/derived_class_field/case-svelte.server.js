import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			width = 0;
			height = 0;
			#area = $.derived(() => this.width * this.height);
			get area() {
				return this.#area();
			}
			set area($$value) {
				return this.#area($$value);
			}
		}
		let box = new Box();
		$$renderer.push(`<p>${$.escape(box.area)}</p>`);
	});
}
