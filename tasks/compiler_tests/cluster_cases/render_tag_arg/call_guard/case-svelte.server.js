import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { children } = $$props;
	let count = 0;
	function label(n) {
		return n + 1;
	}
	$$renderer.push(`<button>go</button> `);
	children($$renderer, label(count));
	$$renderer.push(`<!---->`);
}
