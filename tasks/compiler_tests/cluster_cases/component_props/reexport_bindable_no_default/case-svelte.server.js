import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { stuff = void 0 } = $$props;
		$.bind_props($$props, {
			stuff,
			stuff
		});
	});
}
