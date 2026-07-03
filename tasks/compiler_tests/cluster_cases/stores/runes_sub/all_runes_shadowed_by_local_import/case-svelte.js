import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { state, derived } from "./store.js";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $state = () => $.store_get(state, "$state", $$stores);
	const $derived = () => $.store_get(derived, "$derived", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let a = $state()(0);
	let b = $derived()(0);
	$.init();
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${b ?? ""}`));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
