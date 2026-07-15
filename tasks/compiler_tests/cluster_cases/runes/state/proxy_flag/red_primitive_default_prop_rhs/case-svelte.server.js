import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { flag = false } = $$props;
	let value = 0;
	function apply() {
		value = flag;
	}
	$$renderer.push(`<button>${$.escape(value)}</button>`);
}
