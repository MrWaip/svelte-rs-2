import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $s = () => $.store_get(s, "$s", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const s = writable({
		a: 1,
		b: 2,
		c: 3
	});
	$.legacy_pre_effect(() => $s(), () => {
		({a, ...rest} = $s());
	});
	$.legacy_pre_effect_reset();
	$.init();
	var button = root();
	button.textContent = `${a ?? ""}${$.untrack(() => rest.b) ?? ""}`;
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}
