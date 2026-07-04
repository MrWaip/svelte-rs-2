App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Test {
		#__;
		get "1"() {
			return $.get(this.#__);
		}
		set "1"(value) {
			$.set(this.#__, value, true);
		}
		#_ = $.tag($.state(), "Test.0");
		get 0() {
			return $.get(this.#_);
		}
		set 0(value) {
			$.set(this.#_, value, true);
		}
		constructor() {
			this.#__ = $.tag($.state(), "Test.1");
		}
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
