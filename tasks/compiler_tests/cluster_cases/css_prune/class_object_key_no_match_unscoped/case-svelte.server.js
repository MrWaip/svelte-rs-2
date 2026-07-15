import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { c } = $$props;
	$$renderer.push(`<div${$.attr_class($.clsx({ foo: c }))}>x</div>`);
}
