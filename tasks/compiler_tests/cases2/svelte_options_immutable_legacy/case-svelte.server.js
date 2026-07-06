import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let items = $.fallback($$props["items"], () => [1, 2], true);
		let user = $.fallback($$props["user"], () => ({ name: "a" }), true);
		$$renderer.push(`<p>${$.escape(items.length)}</p> <p>${$.escape(user.name)}</p>`);
		$.bind_props($$props, {
			items,
			user
		});
	});
}
