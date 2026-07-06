App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$store_subs;
		let count = $.fallback($$props["count"], 0);
		let obj = $.fallback($$props["obj"], () => ({ x: 0 }), true);
		let local = 0;
		let localObj = { x: 0 };
		let deep = { a: { b: { c: { x: 0 } } } };
		let key = "x";
		let nestedKey = "b";
		const store = writable(0);
		const objStore = writable({ x: 0 });
		function script_ops() {
			count = 1;
			count += 2;
			count -= 3;
			count++;
			count--;
			++count;
			--count;
			count &&= 4;
			count ||= 5;
			count ??= 6;
			obj.x = 7;
			obj.x += 8;
			obj.x++;
			obj.x--;
			++obj.x;
			--obj.x;
			obj.x &&= 9;
			obj.x ||= 10;
			obj.x ??= 11;
			obj["x"] = 7;
			obj["x"] += 8;
			obj["x"]++;
			++obj["x"];
			obj[key] = 7;
			obj[key] += 8;
			obj[key]++;
			++obj[key];
			local = 12;
			local += 13;
			local -= 14;
			local++;
			local--;
			++local;
			--local;
			local &&= 15;
			local ||= 16;
			local ??= 17;
			localObj.x = 18;
			localObj.x += 19;
			localObj.x++;
			localObj.x--;
			++localObj.x;
			--localObj.x;
			localObj.x &&= 20;
			localObj.x ||= 21;
			localObj.x ??= 22;
			localObj["x"] = 18;
			localObj["x"] += 19;
			localObj["x"]++;
			++localObj["x"];
			localObj[key] = 18;
			localObj[key] += 19;
			localObj[key]++;
			++localObj[key];
			deep.a.b.c.x = 1;
			deep.a.b.c.x += 2;
			deep.a.b.c.x -= 3;
			deep.a.b.c.x++;
			deep.a.b.c.x--;
			++deep.a.b.c.x;
			--deep.a.b.c.x;
			deep.a.b.c.x &&= 4;
			deep.a.b.c.x ||= 5;
			deep.a.b.c.x ??= 6;
			deep["a"]["b"]["c"]["x"] = 1;
			deep["a"]["b"]["c"]["x"] += 2;
			deep["a"]["b"]["c"]["x"]++;
			++deep["a"]["b"]["c"]["x"];
			deep[nestedKey].c[key] = 1;
			deep[nestedKey].c[key] += 2;
			deep[nestedKey].c[key]++;
			++deep[nestedKey].c[key];
			deep.a[nestedKey].c.x = 1;
			deep.a[nestedKey].c.x += 2;
			deep.a[nestedKey].c.x++;
			deep.a.b.c[key] = 1;
			deep.a.b.c[key]++;
			$.store_set(store, 23);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) + 24);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) - 25);
			$.update_store($$store_subs ??= {}, "$store", store);
			$.update_store($$store_subs ??= {}, "$store", store, -1);
			$.update_store_pre($$store_subs ??= {}, "$store", store);
			$.update_store_pre($$store_subs ??= {}, "$store", store, -1);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) && 26);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) || 27);
			$.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) ?? 28);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x = 29);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x += 30);
			$.store_get($$store_subs ??= {}, "$objStore", objStore).x++;
			$.store_get($$store_subs ??= {}, "$objStore", objStore).x--;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore).x;
			--$.store_get($$store_subs ??= {}, "$objStore", objStore).x;
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x &&= 31);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ||= 32);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ??= 33);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] = 29);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] += 30);
			$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"]++;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"];
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] = 29);
			$.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] += 30);
			$.store_get($$store_subs ??= {}, "$objStore", objStore)[key]++;
			++$.store_get($$store_subs ??= {}, "$objStore", objStore)[key];
		}
		$$renderer.push(`<!---->${$.escape(count)}
${$.escape(obj.x)}
${$.escape(local)}
${$.escape(localObj.x)}
${$.escape(deep.a.b.c.x)}
${$.escape(deep["a"]?.["b"]?.["c"]?.["x"])}
${$.escape(deep?.a?.b?.c?.x)}
${$.escape(deep[nestedKey]?.c?.[key])}
${$.escape($.store_get($$store_subs ??= {}, "$store", store))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x)}

${$.escape(count = 1)}
${$.escape(count += 2)}
${$.escape(count -= 3)}
${$.escape(count++)}
${$.escape(count--)}
${$.escape(++count)}
${$.escape(--count)}
${$.escape(count &&= 4)}
${$.escape(count ||= 5)}
${$.escape(count ??= 6)}

${$.escape(obj.x = 7)}
${$.escape(obj.x += 8)}
${$.escape(obj.x++)}
${$.escape(obj.x--)}
${$.escape(++obj.x)}
${$.escape(--obj.x)}
${$.escape(obj.x &&= 9)}
${$.escape(obj.x ||= 10)}
${$.escape(obj.x ??= 11)}
${$.escape(obj["x"] = 7)}
${$.escape(obj["x"] += 8)}
${$.escape(obj["x"]++)}
${$.escape(++obj["x"])}
${$.escape(obj[key] = 7)}
${$.escape(obj[key] += 8)}
${$.escape(obj[key]++)}
${$.escape(++obj[key])}

${$.escape(local = 12)}
${$.escape(local += 13)}
${$.escape(local -= 14)}
${$.escape(local++)}
${$.escape(local--)}
${$.escape(++local)}
${$.escape(--local)}
${$.escape(local &&= 15)}
${$.escape(local ||= 16)}
${$.escape(local ??= 17)}

${$.escape(localObj.x = 18)}
${$.escape(localObj.x += 19)}
${$.escape(localObj.x++)}
${$.escape(localObj.x--)}
${$.escape(++localObj.x)}
${$.escape(--localObj.x)}
${$.escape(localObj.x &&= 20)}
${$.escape(localObj.x ||= 21)}
${$.escape(localObj.x ??= 22)}
${$.escape(localObj["x"] = 18)}
${$.escape(localObj["x"] += 19)}
${$.escape(localObj["x"]++)}
${$.escape(++localObj["x"])}
${$.escape(localObj[key] = 18)}
${$.escape(localObj[key] += 19)}
${$.escape(localObj[key]++)}
${$.escape(++localObj[key])}

${$.escape(deep.a.b.c.x = 1)}
${$.escape(deep.a.b.c.x += 2)}
${$.escape(deep.a.b.c.x -= 3)}
${$.escape(deep.a.b.c.x++)}
${$.escape(deep.a.b.c.x--)}
${$.escape(++deep.a.b.c.x)}
${$.escape(--deep.a.b.c.x)}
${$.escape(deep.a.b.c.x &&= 4)}
${$.escape(deep.a.b.c.x ||= 5)}
${$.escape(deep.a.b.c.x ??= 6)}
${$.escape(deep["a"]["b"]["c"]["x"] = 1)}
${$.escape(deep["a"]["b"]["c"]["x"] += 2)}
${$.escape(deep["a"]["b"]["c"]["x"]++)}
${$.escape(++deep["a"]["b"]["c"]["x"])}
${$.escape(deep[nestedKey].c[key] = 1)}
${$.escape(deep[nestedKey].c[key] += 2)}
${$.escape(deep[nestedKey].c[key]++)}
${$.escape(++deep[nestedKey].c[key])}
${$.escape(deep.a[nestedKey].c.x = 1)}
${$.escape(deep.a[nestedKey].c.x += 2)}
${$.escape(deep.a[nestedKey].c.x++)}
${$.escape(deep.a.b.c[key] = 1)}
${$.escape(deep.a.b.c[key]++)}

${$.escape($.store_set(store, 23))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) + 24))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) - 25))}
${$.escape($.update_store($$store_subs ??= {}, "$store", store))}
${$.escape($.update_store($$store_subs ??= {}, "$store", store, -1))}
${$.escape($.update_store_pre($$store_subs ??= {}, "$store", store))}
${$.escape($.update_store_pre($$store_subs ??= {}, "$store", store, -1))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) && 26))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) || 27))}
${$.escape($.store_set(store, $.store_get($$store_subs ??= {}, "$store", store) ?? 28))}

${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x = 29))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x += 30))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x++)}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore).x--)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore).x)}
${$.escape(--$.store_get($$store_subs ??= {}, "$objStore", objStore).x)}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x &&= 31))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ||= 32))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore).x ??= 33))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] = 29))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)["x"] += 30))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore)["x"]++)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore)["x"])}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] = 29))}
${$.escape($.store_mutate($$store_subs ??= {}, "$objStore", objStore, $.store_get($$store_subs ??= {}, "$objStore", objStore)[key] += 30))}
${$.escape($.store_get($$store_subs ??= {}, "$objStore", objStore)[key]++)}
${$.escape(++$.store_get($$store_subs ??= {}, "$objStore", objStore)[key])} <button>`);
		$.push_element($$renderer, "button", 253, 0);
		$$renderer.push(`run</button>`);
		$.pop_element();
		if ($$store_subs) $.unsubscribe_stores($$store_subs);
		$.bind_props($$props, {
			count,
			obj
		});
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
