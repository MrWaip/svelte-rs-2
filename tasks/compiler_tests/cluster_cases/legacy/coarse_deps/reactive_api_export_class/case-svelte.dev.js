import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	class K {}
	$.legacy_pre_effect(() => $.deep_read_state(a()), () => {
		a();
		K;
	});
	$.legacy_pre_effect_reset();
	var $$exports = {
		...$.legacy_api(),
		get K() {
			return K;
		},
		set K($$value) {
			K = $$value;
		}
	};
	$.bind_prop($$props, "K", K);
	return $.pop($$exports);
}
