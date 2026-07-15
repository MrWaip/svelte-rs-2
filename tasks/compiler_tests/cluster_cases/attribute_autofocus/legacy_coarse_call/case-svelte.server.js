import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let adapter = $$props["adapter"];
		let day = $$props["day"];
		let focused = null;
		function pick() {
			focused = day;
		}
		$$renderer.push(`<button${$.attr("autofocus", focused !== null && adapter.isSame(day, focused), true)}></button>`);
		$.bind_props($$props, {
			adapter,
			day
		});
	});
}
