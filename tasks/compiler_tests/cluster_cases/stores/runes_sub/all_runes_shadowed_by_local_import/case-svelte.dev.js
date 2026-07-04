import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { state, derived } from "./store.js";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const $derived = () => ($.validate_store(derived, "derived"), $.store_get(derived, "$derived", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let a = $state()(0);
	let b = $derived()(0);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
