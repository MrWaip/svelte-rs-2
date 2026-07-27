import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	$.user_effect(() => {
		console.log("before");
	});
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(1))(), () => void $.user_effect(() => {
		console.log("after");
	})]);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
