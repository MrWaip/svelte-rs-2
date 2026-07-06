import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { value } = $$props;
	$$renderer.push(`<p>before</p> <!-- between --> <p>after ${$.escape(value)}</p>`);
}
