App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $page = () => ($.validate_store(page, "page"), $.store_get(page, "$page", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const page = writable(1);
	const value = $.prop($$props, "value", 19, $page);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, text);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
