import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { flag } = $$props;
	let invalid = $.derived(() => Boolean(flag));
	function reset() {
		invalid(false);
	}
	$$renderer.push(`<button>${$.escape(invalid())}</button>`);
}
