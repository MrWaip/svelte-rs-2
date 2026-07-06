import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let counter = 0;
	let active = false;
	function getHandler() {
		return () => counter++;
	}
	$$renderer.push(`<div${$.attr_class($.clsx({ big: counter > 10 }), void 0, { "active": active })}>content</div>`);
}
