import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	Wrap($$renderer, { $$slots: { action: ($$renderer) => {
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "action", {}, () => {
			$$renderer.push(`<div class="action svelte-1nvkc8o">fallback</div>`);
		});
		$$renderer.push(`<!--]-->`);
	} } });
}
