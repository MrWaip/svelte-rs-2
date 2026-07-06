import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	let { maybe } = $$props;
	let handler = $.derived(() => maybe ? (node) => {} : null);
	Inner($$renderer, {});
}
