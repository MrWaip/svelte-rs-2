import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { show = null } = $$props;
	show?.($$renderer, "hello");
	$$renderer.push(`<!---->`);
}
