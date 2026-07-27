import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var d, k, m, y;
	var $$promises = $.run([
		async () => d = (await $.track_reactivity_loss(fetch("/a")))(),
		() => k = 5,
		async () => m = (await $.track_reactivity_loss(fetch("/b")))(),
		() => y = 1
	]);
	var $$exports = {
		...$.legacy_api(),
		get k() {
			return k;
		},
		set k($$value) {
			k = $$value;
		}
	};
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${d ?? ""}${k ?? ""}${m ?? ""}1`), void 0, void 0, [
		$$promises[0],
		$$promises[1],
		$$promises[2],
		$$promises[3]
	]);
	$.append($$anchor, text);
	return $.pop($$exports);
}
