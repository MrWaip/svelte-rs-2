import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let absolute = void 0;
	let visible = void 0;
	let unchanged = void 0;
	let untouched = void 0;
	const staticClass = true;
	visible = 12;
	absolute = true;
	$$renderer.push(`<div${$.attr_class("", void 0, {
		"visible": visible,
		"absolute": absolute,
		"unchanged": unchanged,
		"untouched": untouched,
		"staticClass": staticClass,
		"static2": true
	})}>Lorem</div>`);
}
