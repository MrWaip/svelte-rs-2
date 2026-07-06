import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let variant = "primary";
	function badge($$renderer, text) {
		$$renderer.push(`<span${$.attr_class("badge svelte-yczv4j", void 0, { "primary": variant === "primary" })}>${$.escape(text)}</span>`);
	}
	badge($$renderer, "hi");
}
