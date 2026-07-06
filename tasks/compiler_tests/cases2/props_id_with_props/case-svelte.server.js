import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const id = $.props_id($$renderer);
	let { name } = $$props;
	$$renderer.push(`<div${$.attr("id", id)}>${$.escape(name)}</div>`);
}
