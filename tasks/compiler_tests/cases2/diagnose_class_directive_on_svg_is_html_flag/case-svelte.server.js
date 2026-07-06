import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { active = false } = $$props;
	$$renderer.push(`<svg${$.attr_class("icon", void 0, { "active": active })}><path d="M0 0"></path></svg>`);
}
