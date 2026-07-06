import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	Wrapper($$renderer, { $$slots: { cell: ($$renderer, { week }) => {
		$$renderer.push(`<!--[-->`);
		$.slot($$renderer, $$props, "cell", { week }, null);
		$$renderer.push(`<!--]-->`);
	} } });
}
