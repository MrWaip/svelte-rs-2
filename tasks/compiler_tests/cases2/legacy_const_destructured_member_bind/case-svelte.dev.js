import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function makePayload() {
		return { payoffLazy: { data: null } };
	}
	const tmp = makePayload(), payoffLazy = $.tag($.mutable_source(tmp.payoffLazy), "payoffLazy");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		get payoffStore() {
			return $.get(payoffLazy).data;
		},
		set payoffStore($$value) {
			$.mutate(payoffLazy, $.get(payoffLazy).data = $$value);
		},
		$$legacy: true
	}), "component", App, 11, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
