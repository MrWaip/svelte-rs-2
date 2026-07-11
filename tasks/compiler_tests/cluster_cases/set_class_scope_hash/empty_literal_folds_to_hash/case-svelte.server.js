import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { on } = $$props;
	$$renderer.push(`<div${$.attr_class("", void 0, { "active": on })}>a</div>`);
}
