import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.push(`<p>before</p> <!--[-->`);
	$.slot($$renderer, $$props, "footer", {}, () => {
		$$renderer.push(`<span>fallback</span>`);
	});
	$$renderer.push(`<!--]--> <p>after</p>`);
}
