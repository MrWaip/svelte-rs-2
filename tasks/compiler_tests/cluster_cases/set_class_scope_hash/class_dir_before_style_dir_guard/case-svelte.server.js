import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { on } = $$props;
	$$renderer.push(`<div${$.attr_class("svelte-lvqw8l", void 0, { "active": on })}${$.attr_style("", { color: "red" })}>a</div>`);
}
