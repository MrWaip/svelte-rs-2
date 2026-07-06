import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let user = $.fallback($$props["user"], () => ({ fetch: () => Promise.resolve(null) }), true);
		$.await($$renderer, user.fetch(), () => {}, (v) => {
			$$renderer.push(`<span>${$.escape(v)}</span>`);
		});
		$$renderer.push(`<!--]-->`);
		$.bind_props($$props, { user });
	});
}
