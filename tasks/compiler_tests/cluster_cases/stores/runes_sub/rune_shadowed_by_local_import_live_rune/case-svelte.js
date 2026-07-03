import * as $ from "svelte/internal/client";
import { state } from "./store.js";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let a = $state()(0);
	let b = $.derived(() => 0);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${a ?? ""} ${$.get(b) ?? ""}`));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
