import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $search = () => ($.validate_store(search, "search"), $.store_get(search, "$search", $$stores));
	const $error = () => ($.validate_store(error, "error"), $.store_get(error, "$error", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const search = writable({ value: "" });
	const error = writable("");
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		let $0 = $.derived_safe_equal(() => ($error(), $.untrack(() => Boolean($error()))));
		$.slot(node, $$props, "default", {
			get value() {
				return $search(), $.untrack(() => $search().value);
			},
			get isInvalid() {
				return $.get($0);
			}
		}, null);
	}
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}
