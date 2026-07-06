import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value } = $$props;
	$$renderer.push(`<div${$.attr_class($.clsx(["container", value]), "svelte-wx745y")}>x</div>`);
}
