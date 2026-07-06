import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, onClick, name } = $$props;
	$$renderer.push(`<div${$.attr_class("", void 0, {
		"x": a,
		"y": Boolean(onClick)
	})}>${$.escape(name)}</div>`);
}
