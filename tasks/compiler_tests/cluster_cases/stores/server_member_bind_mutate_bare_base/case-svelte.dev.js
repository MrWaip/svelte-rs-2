App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $data = () => ($.validate_store($.get(data), "data"), $.store_get($.get(data), "$data", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const data = $.tag($.derived(() => $$props.store.data), "data");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return $data().amount;
		},
		set value($$value) {
			$.store_mutate($.get(data), $.untrack($data).amount = $$value, $.untrack($data));
		}
	}), "component", App, 7, 0, { componentTag: "Child" });
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
