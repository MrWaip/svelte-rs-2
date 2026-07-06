import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = false;
	$$renderer.push(`<div${$.attr_class($.clsx(["container", { active }]), "svelte-wx745y")}>x</div>`);
}
