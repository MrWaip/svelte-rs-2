import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { count, label } = $$props;
	$$renderer.push(`<p>${$.escape(label)}: ${$.escape(count)}</p>`);
}
