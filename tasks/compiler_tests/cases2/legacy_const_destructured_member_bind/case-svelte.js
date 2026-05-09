import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor) {
	function makePayload() {
		return { payoffLazy: { data: null } };
	}
	const tmp = makePayload(), payoffLazy = $.mutable_source(tmp.payoffLazy);
	Child($$anchor, {
		get payoffStore() {
			return $.get(payoffLazy).data;
		},
		set payoffStore($$value) {
			$.mutate(payoffLazy, $.get(payoffLazy).data = $$value);
		},
		$$legacy: true
	});
}
