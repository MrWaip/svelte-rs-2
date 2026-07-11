import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { obj = void 0 } = $$props;
		function sync() {
			obj.field = obj.other;
		}
		$$renderer.push(`<button>go</button>`);
		$.bind_props($$props, { obj });
	});
}
