import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#a = { val: 0 };
			#b = 0;
			mix() {
				this.#a ??= { val: 1 };
				this.#b += 1;
			}
			get a() {
				return this.#a?.val;
			}
			get b() {
				return this.#b;
			}
		}
		const box = new Box();
		$$renderer.push(`<p>${$.escape(box.a)} ${$.escape(box.b)}</p>`);
	});
}
