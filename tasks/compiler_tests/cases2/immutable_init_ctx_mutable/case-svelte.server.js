import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let user = $.fallback($$props["user"], () => ({ name: "a" }), true);
		$$renderer.push(`<p>${$.escape(user.name)}</p>`);
		$.bind_props($$props, { user });
	});
}
