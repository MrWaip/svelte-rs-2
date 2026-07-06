import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let x = 0;
	function getValues() {
		const doubled = $.derived(() => x * 2);
		return {
			doubled: doubled(),
			get live() {
				return doubled();
			}
		};
	}
	$.bind_props($$props, { getValues });
}
