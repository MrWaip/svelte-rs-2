import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let cls = $$props["cls"];
	$$renderer.push(`<my-element${$.attr_class($.clsx(cls))}></my-element>`);
	$.bind_props($$props, { cls });
}
