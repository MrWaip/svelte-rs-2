import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rest = $.fallback($$props["rest"], () => ({}), true);
	function action(node) {}
	$$renderer.push(`<div${$.attributes({ ...rest })}></div>`);
	$.bind_props($$props, { rest });
}
