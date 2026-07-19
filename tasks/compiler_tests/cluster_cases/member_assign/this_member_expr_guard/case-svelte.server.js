import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let count = 0;
		function action(node) {
			return { update(count) {
				console.log("update", this.count, this.count = count);
			} };
		}
		$$renderer.push(`<button>${$.escape(count)}</button>`);
	});
}
