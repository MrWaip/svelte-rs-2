import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { cls } = $$props;
	const active = cls === "on";
	$$renderer.push(`<div${$.attr_class($.clsx(active ? "a" : "b"))}>${$.escape(active)}</div>`);
}
