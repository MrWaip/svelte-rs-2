import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { title } = $$props;
	$$renderer.push(`<div${$.attr("title", title)}></div>`);
}
