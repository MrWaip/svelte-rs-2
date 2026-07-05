import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	function createCounter() {
		let count = 0;
		return {
			get count() {
				return count;
			},
			increment() {
				count++;
			}
		};
	}
	$.bind_props($$props, { createCounter });
}
