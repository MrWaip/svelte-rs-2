import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $toolbar = () => ($.validate_store(toolbar, "toolbar"), $.store_get(toolbar, "$toolbar", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const toolbar = writable({ title: "" });
	function getTitle(t) {
		return t;
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(($0) => {
			$.document.title = $0 ?? "";
		}, [() => ($toolbar(), $.untrack(() => getTitle($toolbar().title)))]);
	});
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
