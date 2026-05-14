import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $source = () => $.store_get(source, "$source", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const actions = $.mutable_source();
	function pick(value) {
		return value;
	}
	const source = writable(0);
	$.legacy_pre_effect(() => $source(), () => {
		$.set(actions, pick($source()));
	});
	$.legacy_pre_effect_reset();
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(actions)));
	$.append($$anchor, text);
	$.pop();
	$$cleanup();
}
