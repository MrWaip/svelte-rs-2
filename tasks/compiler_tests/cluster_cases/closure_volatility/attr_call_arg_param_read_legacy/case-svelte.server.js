import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let a = $$props["a"];
	let count = 0;
	function bump() {
		count += 1;
	}
	$$renderer.push(`<div${$.attr("title", String((x) => x))}>${$.escape(a)}</div> <button>${$.escape(count)}</button>`);
	$.bind_props($$props, { a });
}
