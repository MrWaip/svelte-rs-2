import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	$$renderer.push(`<div${$.attr_class(`title`, "svelte-95v2c0", { "active": x })}>a</div>`);
}
