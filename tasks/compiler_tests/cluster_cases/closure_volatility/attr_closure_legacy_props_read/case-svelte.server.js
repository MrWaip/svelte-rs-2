import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const $$sanitized_props = $.sanitize_props($$props);
	$$renderer.component(($$renderer) => {
		let a = $$props["a"];
		let count = 0;
		function bump() {
			count += 1;
		}
		$$renderer.push(`<div${$.attr("title", [() => $$sanitized_props.x])}>${$.escape(a)}</div> <button>${$.escape(count)}</button>`);
		$.bind_props($$props, { a });
	});
}
