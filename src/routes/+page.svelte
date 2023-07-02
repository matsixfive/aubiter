<script lang="ts">
	import init, { Universe, Output, Vector3 } from "gravity";
	// we need onMount to run init
	import { onMount } from "svelte";

	const buildArray = (output: Output) => {
		const planets = [];
		for (let i = 0; i < output.length; i++) {
			const planet = output.get_planet(i);
			planets.push({
				name: planet.name,
				radius: planet.radius,
				velocity: planet.velocity,
				position: planet.position,
			});
		}
		return planets;
	};

	const colors = {
		earth: "00ff00",
		mars: "ff0000",
		moon: "0a0a0a",
		sun: "eadf00",
	};

	let output: {
		name: string;
		radius: number;
		velocity: Vector3;
		position: Vector3;
	}[];

	let speedUp = () => {};
	let speedDown = () => {};
	let stop = () => {};
	let setSpeed = (speed:number) => {};
	let addPlanet = () => {};
	let tick = (amount: number) => {};
	let speed = 1;
	let tps = 0

	let sunpos = [0, 0, 0];

	let framerate = 60; // temp inital value
	let displayFramerate = framerate;

	onMount(async () => {
		await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS

		const universe = new Universe;
		universe.speed = speed;

		// universe.add_planet("earth", new Vector3(152.02e9, 0, 0), new Vector3(0, -29951.68, 0), 5.972e24, 6371e3);
		// universe.add_planet("sun", new Vector3(0, 0, 0), new Vector3(0, 0, 0), 1.989e30, 696340e3);

		universe.add_planet("planet1", new Vector3(0, 9e7, 0), new Vector3(0, 0, 0), 1e27, 2e7);
		universe.add_planet("planet2", new Vector3(0, -9e7, 0), new Vector3(-1e4, 0, 0), 1e27, 2e7);

		speedUp = () => {
			if (speed === 0 ) {
				setSpeed(speed = 1);
			} else {
				setSpeed(speed = Math.ceil(speed * 1.25));
			}
		};
		speedDown = () => {
			setSpeed(speed = Math.floor(speed / 1.25));
		};
		stop = () => {
			setSpeed(speed = 0);
		};
		setSpeed = (speed: number) => {
			if (speed < 0) return;
			universe.speed = speed;
			tps = speed * framerate;
		};
		addPlanet = () => {
			universe.add_planet(Date.now().toString(), new Vector3(0, 384400, 0), new Vector3(0, 0, 0), 6.39e13, 3.389e6);
		};
		tick = (amount: number) => {
			const speed = universe.speed;
			setSpeed(amount);
			universe.tick();
			const render = universe.render();
			output = buildArray(render);
			setSpeed(speed);
		};
	
		let lastDate = performance.now();
		let loop = () => {
			universe.tick();

			const render = universe.render();
			output = buildArray(render);

			framerate = 1000 / (performance.now() - lastDate);
			lastDate = performance.now();

			// ---
			for (let i = 0; i < output.length; i++) {
				const planet = output[i];
				if (planet.name === "sun") {
					sunpos = [planet.position.x, planet.position.y, planet.position.z];
					break;
				}
			}
			// ---

			requestAnimationFrame(loop);
		};

		requestAnimationFrame(loop);

		let interval = setInterval(() => {
			displayFramerate = framerate;
			tps = speed * framerate;
		}, 500);

		return () => {
			universe.free();
			clearInterval(interval);
			loop = () => {};
		};
	});

	$: speed = Math.round(speed * 100) / 100;
	$: setSpeed(speed);

	let zoom = 300000;
	let sizeZoom = 1;
</script>

<div>
	{#if output}
		<div style="position:relative; width:100vmin;height:100vmin">
			{Math.round(displayFramerate)}fps
			<br />
			<!-- button on:click={addPlanet}>Add Planet</button -->
			<!-- br / -->
			<button on:click={() => tick(1)}>Tick x1</button>
			<button on:click={() => tick(10)}>Tick x10</button>
			<button on:click={() => tick(100)}>Tick x100</button>
			<button on:click={() => tick(1000)}>Tick x1000</button>
			<br />
			{#each output as planet}
				<div>{planet.name}: ({Math.round(Math.pow((Math.pow(planet.velocity.x, 2) + Math.pow(planet.velocity.y, 2)), 0.5))})</div>
			{/each}
			<button on:click={stop}>pause</button>
			<button on:click={speedDown}>{"<<"}</button>
				<input bind:value={speed} type="number"/>x
			<button on:click={speedUp}>{">>"}</button>
			{Math.ceil(tps).toLocaleString()}tps

			<div
				style={`position:absolute; border-radius:50%; transform:translate(-50%, -50%);top:50%; left:50%;width:${2}px;height:${2}px;background-color:black;`}
			/>
			{#each output as planet}
				<div
					data-name={planet.name}
					style={`position:absolute; transform:translate(calc(-50% + ${
						(planet.position.x - sunpos[0] ) / zoom
						// planet.position.x / zoom	
					}px), calc(-50% - ${
						(planet.position.y - sunpos[1])/ zoom
						// planet.position.y() / zoom
					}px)); border-radius:50%; top:50%; left:50%;width:${
						planet.radius / (zoom / 2) * sizeZoom
					}px;height:${
						planet.radius / (zoom / 2) * sizeZoom
					}px;background-color:#${
						colors.hasOwnProperty(planet.name) ? colors[planet.name] : "aaaaaa"
					};`}
				/>
			{/each}
		</div>
	{/if}
</div>
