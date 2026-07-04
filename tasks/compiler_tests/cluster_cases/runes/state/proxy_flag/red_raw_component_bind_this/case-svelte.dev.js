App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let elem = $.tag($.state(void 0), "elem");
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", $.get(elem)));
	});
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => Inner, ($$anchor, $$component) => {
		$.bind_this($$component($$anchor, {}), ($$value) => $.set(elem, $$value), () => $.get(elem));
	}), "component", App, 9, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
