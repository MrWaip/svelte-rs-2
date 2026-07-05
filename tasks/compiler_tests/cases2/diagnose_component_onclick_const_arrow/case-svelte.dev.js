App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Btn from "./Btn.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const save = () => {};
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Btn($$anchor, { onclick: save }), "component", App, 6, 0, { componentTag: "Btn" });
	return $.pop($$exports);
}
