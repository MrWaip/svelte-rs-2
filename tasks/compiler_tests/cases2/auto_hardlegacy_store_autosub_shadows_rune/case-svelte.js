import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const state = writable({ x: 0 });
	let y = $.mutable_source(0);
	$.legacy_pre_effect(() => $state(), () => {
		$.set(y, $state().x);
	});
	$.legacy_pre_effect_reset();
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(y)));
	$.append($$anchor, text);
	$.pop();
	$$cleanup();
}
