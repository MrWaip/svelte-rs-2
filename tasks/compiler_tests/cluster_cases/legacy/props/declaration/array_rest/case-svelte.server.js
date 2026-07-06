import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let tmp = [
			1,
			2,
			3
		], $$array = $.to_array(tmp), a = $.fallback($$props["a"], () => $$array[0], true), rest = $.fallback($$props["rest"], () => $$array.slice(1), true);
		$$renderer.push(`<button>${$.escape(a)}${$.escape(rest.length)}</button>`);
		$.bind_props($$props, {
			a,
			rest
		});
	});
}
