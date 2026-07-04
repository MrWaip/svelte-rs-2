App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Modal from "./Modal.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Modal($$anchor, {}), "component", App, 5, 0, { componentTag: "Modal" });
	return $.pop($$exports);
}
