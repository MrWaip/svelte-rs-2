import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $error = () => ($.validate_store(error(), "error"), $.store_get(error(), "$error", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let error = $.prop($$props, "error", 8);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			$.mark_store_binding();
			return $error();
		},
		set value($$value) {
			$.store_set(error(), $$value);
		},
		$$legacy: true
	}), "component", App, 6, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
