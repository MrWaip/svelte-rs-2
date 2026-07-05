App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $value = () => ($.validate_store($$props.value, "value"), $.store_get($$props.value, "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get cents() {
			$.mark_store_binding();
			return $value();
		},
		set cents($$value) {
			$.store_set($$props.value, $$value);
		}
	}), "component", App, 6, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
