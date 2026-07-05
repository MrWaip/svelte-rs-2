import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $source = () => ($.validate_store(source, "source"), $.store_get(source, "$source", $$stores));
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
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, $.get(actions)));
	$.append($$anchor, text);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
