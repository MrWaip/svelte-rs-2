import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $derived = () => ($.validate_store(derived, "derived"), $.store_get(derived, "$derived", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let source = { value: 1 };
	let { value } = $derived()(source);
	var $$exports = { ...$.legacy_api() };
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
