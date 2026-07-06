import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value: local = { stats: { count: 0 } } } = $$props;
		let key = "count";
		function bump() {
			local.stats[key]++;
		}
	});
}
