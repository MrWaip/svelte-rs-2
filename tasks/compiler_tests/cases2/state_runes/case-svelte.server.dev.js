App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { debounce } from "es-toolkit";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let value = "long string value";
		let value2 = undefined;
		let value3 = void 0;
		let value4 = {};
		let value5 = value1;
		let value6 = null;
		let value7 = () => {};
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
