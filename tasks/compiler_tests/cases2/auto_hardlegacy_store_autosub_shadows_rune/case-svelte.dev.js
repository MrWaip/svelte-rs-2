import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const state = writable({ x: 0 });
	let y = $.tag($.mutable_source(0), "y");
	$.legacy_pre_effect(() => $state(), () => {
		$.set(y, $state().x);
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(y)));
	$.append($$anchor, text);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
