App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const data = 0;
const derived = $.tag($.derived(() => data + 1), "derived");
const state = {
	data,
	derived: $.get(derived)
};
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $state = () => ($.validate_store(state, "state"), $.store_get(state, "$state", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	$.inspect(() => [state], (...$$args) => console.log(...$$args), true);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, state.data));
	$.append($$anchor, text);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
