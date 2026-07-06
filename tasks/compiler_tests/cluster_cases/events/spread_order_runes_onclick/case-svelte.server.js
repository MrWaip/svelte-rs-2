import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest } = $$props;
	function f() {
		return () => {};
	}
	$$renderer.push(`<div${$.attributes({ ...rest })}></div>`);
}
