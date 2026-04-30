<script>
	import { writable } from 'svelte/store';

	let { prop = 0, propObj = { x: 0 }, bind = $bindable(0), bindObj = $bindable({ x: 0 }) } = $props();

	let count = $state(0);
	let countObj = $state({ x: 0 });
	let raw = $state.raw(0);
	let deep = $state({ a: { b: { c: { x: 0 } } } });
	let key = 'x';
	let nestedKey = 'b';

	const store = writable(0);
	const objStore = writable({ x: 0 });

	let items = $state([{ id: 1, x: 0 }, { id: 2, x: 0 }]);
	let promise = Promise.resolve({ x: 0 });

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

		countObj.x = 7;
		countObj.x += 8;
		countObj.x++;
		countObj.x--;
		++countObj.x;
		--countObj.x;
		countObj.x &&= 9;
		countObj.x ||= 10;
		countObj.x ??= 11;
		countObj["x"] = 7;
		countObj["x"] += 8;
		countObj["x"]++;
		++countObj["x"];
		countObj[key] = 7;
		countObj[key] += 8;
		countObj[key]++;
		++countObj[key];

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

		raw = 12;
		raw += 13;
		raw -= 14;
		raw++;
		raw--;
		++raw;
		--raw;
		raw &&= 15;
		raw ||= 16;
		raw ??= 17;

		prop = 18;
		prop += 19;
		prop++;
		prop--;
		++prop;
		--prop;
		prop &&= 20;
		prop ||= 21;
		prop ??= 22;

		propObj.x = 23;
		propObj.x += 24;
		propObj.x++;
		propObj.x--;
		++propObj.x;
		--propObj.x;
		propObj.x &&= 25;
		propObj.x ||= 26;
		propObj.x ??= 27;
		propObj["x"] = 23;
		propObj["x"] += 24;
		propObj["x"]++;
		++propObj["x"];
		propObj[key] = 23;
		propObj[key] += 24;
		propObj[key]++;
		++propObj[key];

		bind = 28;
		bind += 29;
		bind++;
		bind--;
		++bind;
		--bind;
		bind &&= 30;
		bind ||= 31;
		bind ??= 32;

		bindObj.x = 33;
		bindObj.x += 34;
		bindObj.x++;
		bindObj.x--;
		++bindObj.x;
		--bindObj.x;
		bindObj.x &&= 35;
		bindObj.x ||= 36;
		bindObj.x ??= 37;
		bindObj["x"] = 33;
		bindObj["x"] += 34;
		bindObj["x"]++;
		++bindObj["x"];
		bindObj[key] = 33;
		bindObj[key] += 34;
		bindObj[key]++;
		++bindObj[key];

		$store = 38;
		$store += 39;
		$store++;
		$store--;
		++$store;
		--$store;
		$store &&= 40;
		$store ||= 41;
		$store ??= 42;

		$objStore.x = 43;
		$objStore.x += 44;
		$objStore.x++;
		$objStore.x--;
		++$objStore.x;
		--$objStore.x;
		$objStore.x &&= 45;
		$objStore.x ||= 46;
		$objStore.x ??= 47;
		$objStore["x"] = 43;
		$objStore["x"] += 44;
		$objStore["x"]++;
		++$objStore["x"];
		$objStore[key] = 43;
		$objStore[key] += 44;
		$objStore[key]++;
		++$objStore[key];
	}
</script>

{count}
{countObj.x}
{raw}
{deep.a.b.c.x}
{deep["a"]?.["b"]?.["c"]?.["x"]}
{deep?.a?.b?.c?.x}
{deep[nestedKey]?.c?.[key]}
{prop}
{propObj.x}
{bind}
{bindObj.x}
{$store}
{$objStore.x}

{count = 1}
{count += 2}
{count -= 3}
{count++}
{count--}
{++count}
{--count}
{count &&= 4}
{count ||= 5}
{count ??= 6}

{countObj.x = 7}
{countObj.x += 8}
{countObj.x++}
{countObj.x--}
{++countObj.x}
{--countObj.x}
{countObj.x &&= 9}
{countObj.x ||= 10}
{countObj.x ??= 11}
{countObj["x"] = 7}
{countObj["x"] += 8}
{countObj["x"]++}
{++countObj["x"]}
{countObj[key] = 7}
{countObj[key] += 8}
{countObj[key]++}
{++countObj[key]}

{deep.a.b.c.x = 1}
{deep.a.b.c.x += 2}
{deep.a.b.c.x -= 3}
{deep.a.b.c.x++}
{deep.a.b.c.x--}
{++deep.a.b.c.x}
{--deep.a.b.c.x}
{deep.a.b.c.x &&= 4}
{deep.a.b.c.x ||= 5}
{deep.a.b.c.x ??= 6}
{deep["a"]["b"]["c"]["x"] = 1}
{deep["a"]["b"]["c"]["x"] += 2}
{deep["a"]["b"]["c"]["x"]++}
{++deep["a"]["b"]["c"]["x"]}
{deep[nestedKey].c[key] = 1}
{deep[nestedKey].c[key] += 2}
{deep[nestedKey].c[key]++}
{++deep[nestedKey].c[key]}
{deep.a[nestedKey].c.x = 1}
{deep.a[nestedKey].c.x += 2}
{deep.a[nestedKey].c.x++}
{deep.a.b.c[key] = 1}
{deep.a.b.c[key]++}

{raw = 12}
{raw += 13}
{raw -= 14}
{raw++}
{raw--}
{++raw}
{--raw}
{raw &&= 15}
{raw ||= 16}
{raw ??= 17}

{prop = 18}
{prop += 19}
{prop++}
{prop--}
{++prop}
{--prop}
{prop &&= 20}
{prop ||= 21}
{prop ??= 22}

{propObj.x = 23}
{propObj.x += 24}
{propObj.x++}
{propObj.x--}
{++propObj.x}
{--propObj.x}
{propObj.x &&= 25}
{propObj.x ||= 26}
{propObj.x ??= 27}
{propObj["x"] = 23}
{propObj["x"] += 24}
{propObj["x"]++}
{++propObj["x"]}
{propObj[key] = 23}
{propObj[key] += 24}
{propObj[key]++}
{++propObj[key]}

{bind = 28}
{bind += 29}
{bind++}
{bind--}
{++bind}
{--bind}
{bind &&= 30}
{bind ||= 31}
{bind ??= 32}

{bindObj.x = 33}
{bindObj.x += 34}
{bindObj.x++}
{bindObj.x--}
{++bindObj.x}
{--bindObj.x}
{bindObj.x &&= 35}
{bindObj.x ||= 36}
{bindObj.x ??= 37}
{bindObj["x"] = 33}
{bindObj["x"] += 34}
{bindObj["x"]++}
{++bindObj["x"]}
{bindObj[key] = 33}
{bindObj[key] += 34}
{bindObj[key]++}
{++bindObj[key]}

{$store = 38}
{$store += 39}
{$store++}
{$store--}
{++$store}
{--$store}
{$store &&= 40}
{$store ||= 41}
{$store ??= 42}

{$objStore.x = 43}
{$objStore.x += 44}
{$objStore.x++}
{$objStore.x--}
{++$objStore.x}
{--$objStore.x}
{$objStore.x &&= 45}
{$objStore.x ||= 46}
{$objStore.x ??= 47}
{$objStore["x"] = 43}
{$objStore["x"] += 44}
{$objStore["x"]++}
{++$objStore["x"]}
{$objStore[key] = 43}
{$objStore[key] += 44}
{$objStore[key]++}
{++$objStore[key]}

{#each items as item, i (item.id)}
	{item}
	{i}
	{item.x}
	{item.x = 1}
	{item.x += 2}
	{item.x -= 3}
	{item.x++}
	{item.x--}
	{++item.x}
	{--item.x}
	{item.x &&= 4}
	{item.x ||= 5}
	{item.x ??= 6}
	{item["x"] = 1}
	{item["x"] += 2}
	{item["x"]++}
	{++item["x"]}
	{item[key] = 1}
	{item[key] += 2}
	{item[key]++}
	{++item[key]}
	<button onclick={() => {
		item.x = 1;
		item.x += 2;
		item.x++;
		++item.x;
		item.x &&= 4;
		item.x ||= 5;
		item.x ??= 6;
		item["x"] = 1;
		item[key] = 1;
		item[key]++;
	}}>each-row</button>
{/each}

{#snippet card(param)}
	{param}
	{param.x}
	{param.x = 1}
	{param.x += 2}
	{param.x++}
	{++param.x}
	{param.x &&= 3}
	{param.x ||= 4}
	{param.x ??= 5}
	{param["x"] = 1}
	{param["x"]++}
	{param[key] = 1}
	{param[key]++}
	<button onclick={() => {
		param.x = 1;
		param.x += 2;
		param.x++;
		++param.x;
		param.x &&= 3;
		param.x ||= 4;
		param.x ??= 5;
		param[key] = 1;
		param[key]++;
	}}>snippet</button>
{/snippet}

{#each items as it (it.id)}
	{@const ctx = it}
	{ctx.x}
	{ctx.x = 1}
	{ctx.x += 2}
	{ctx.x++}
	{++ctx.x}
	{ctx.x &&= 3}
	{ctx.x ||= 4}
	{ctx.x ??= 5}
	{ctx["x"] = 1}
	{ctx[key] = 1}
{/each}

{#await promise then v}
	{v.x}
	{v.x = 1}
	{v.x += 2}
	{v.x++}
	{++v.x}
	{v.x &&= 3}
	{v.x ||= 4}
	{v.x ??= 5}
	{v["x"] = 1}
	{v[key] = 1}
{:catch e}
	{e.message}
	{e.x = 1}
	{e.x += 2}
	{e.x++}
	{++e.x}
	{e["x"] = 1}
	{e[key] = 1}
{/await}

<button onclick={script_ops}>run</button>
