import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p class="svelte-sw3owg"> </p>`);
const $$css = {
	hash: "svelte-sw3owg",
	code: "p.svelte-sw3owg {color:red;}"
};
export default function App($$anchor, $$props) {
	$.push($$props, true);
	$.append_styles($$anchor, $$css);
	const $store = () => $.store_get(store, "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = writable(0);
	let count = 0;
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$store() ?? ""} 0`));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
