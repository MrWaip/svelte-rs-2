import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $toolbar = () => $.store_get(toolbar, "$toolbar", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const toolbar = writable({ title: "" });
	function getTitle(t) {
		return t;
	}
	$.init();
	$.head("q2w0q4", ($$anchor) => {
		$.deferred_template_effect(($0) => {
			$.document.title = $0 ?? "";
		}, [() => ($toolbar(), $.untrack(() => getTitle($toolbar().title)))]);
	});
	$.pop();
	$$cleanup();
}
