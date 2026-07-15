import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { c } = $$props;
	$$renderer.push(`<div${$.attr_class($.clsx({ foo: c }), "svelte-11wl0bt")}>x</div>`);
}
