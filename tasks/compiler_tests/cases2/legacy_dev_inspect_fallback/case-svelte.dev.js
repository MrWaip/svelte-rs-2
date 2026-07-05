import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $inspect = () => ($.validate_store(inspect, "inspect"), $.store_get(inspect, "$inspect", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let a = 1;
	let b = 2;
	$inspect()(a, b);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	text.nodeValue = "12";
	$.append($$anchor, text);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
