import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x = "foo" } = $$props;
	const cond = x === "foo";
	$$renderer.push(`<div${$.attr_class(`a ${$.stringify(cond && "b")}`)}></div>`);
}
