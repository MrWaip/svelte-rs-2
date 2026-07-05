import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let className = $.fallback($$props["class"], "btn");
	function getClass() {
		return className;
	}
	$$renderer.push(`<p>${$.escape(className)}</p>`);
	$.bind_props($$props, {
		class: className,
		getClass
	});
}
