import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = writable({});
	$.legacy_pre_effect(() => $s(), () => {
		({p: {a} = {}} = $s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	button.textContent = a;
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
