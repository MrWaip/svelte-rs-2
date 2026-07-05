import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x = 0 } = $$props;
	function fmt(n) {
		return String(n);
	}
	$$renderer.push(`<div${$.attr_class("", void 0, { "active": x === 0 })}><span>${$.escape(fmt(x))}</span></div>`);
}
