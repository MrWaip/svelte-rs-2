App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let currentTime = 0;
		let paused = true;
		let volume = 1;
		let muted = false;
		let playbackRate = 1;
		$$renderer.push(`<audio>`);
		$.push_element($$renderer, "audio", 9, 0);
		$$renderer.push(`</audio>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
