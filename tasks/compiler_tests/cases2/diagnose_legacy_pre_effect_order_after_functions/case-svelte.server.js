import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let maxLength;
		let step = $$props["step"];
		function noop() {}
		$: maxLength = step.maxLength ?? Infinity;
		$.bind_props($$props, { step });
	});
}
