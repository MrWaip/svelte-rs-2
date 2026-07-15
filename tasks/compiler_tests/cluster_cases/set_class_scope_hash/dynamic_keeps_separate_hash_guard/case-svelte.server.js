import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cls, on } = $$props;
	$$renderer.push(`<div${$.attr_class($.clsx(cls), "svelte-c546ri", { "active": on })}>a</div>`);
}
