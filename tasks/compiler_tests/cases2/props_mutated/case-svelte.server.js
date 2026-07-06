import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { count = 0 } = $$props;
	function increment() {
		count++;
	}
	$$renderer.push(`<p>${$.escape(count)}</p>`);
}
