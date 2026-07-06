import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			value = 0;
			bump() {
				this.value++;
			}
		}
		const box = new Box();
		$$renderer.push(`<button>${$.escape(box.value)}</button>`);
	});
}
