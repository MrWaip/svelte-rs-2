import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!-- svelte-ignore a11y_no_static_element_interactions --> <div>click</div>`);
}
