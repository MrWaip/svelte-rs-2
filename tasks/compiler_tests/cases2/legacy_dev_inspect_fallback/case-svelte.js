import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	const $inspect = () => $.store_get(inspect, "$inspect", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let a = 1;
	let b = 2;
	$inspect()(a, b);
	$.next();
	var text = $.text();
	text.nodeValue = "12";
	$.append($$anchor, text);
	$$cleanup();
}
