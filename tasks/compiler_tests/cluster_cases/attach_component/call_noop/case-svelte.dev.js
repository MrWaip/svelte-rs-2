App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let message = "one";
	function attachment(message) {
		return (node) => {
			node.textContent = message;
		};
	}
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Inner($$anchor, { [$.attachment()]: ($$node) => (attachment(message) || $.noop)($$node) }), "component", App, 13, 0, { componentTag: "Inner" });
	return $.pop($$exports);
}
