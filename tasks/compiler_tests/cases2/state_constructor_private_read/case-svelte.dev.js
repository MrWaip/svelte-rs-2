App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Timer {
		#elapsed = $.tag($.state(0), "Timer.#elapsed");
		tick() {
			$.set(this.#elapsed, $.get(this.#elapsed) + 1);
		}
		get display() {
			return $.get(this.#elapsed);
		}
	}
	let t = new Timer();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, t.display));
	$.append($$anchor, p);
	return $.pop($$exports);
}
