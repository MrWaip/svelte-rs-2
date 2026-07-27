import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = 0;
		constructor(initial) {
			this.#count = initial;
		}
	}
	$.pop();
}
