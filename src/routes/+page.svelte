<script lang="ts">
  import init, { Universe, Output } from "gravity";
  // we need onMount to run init
  import { onMount } from "svelte";

	const buildArray = (output: Output) => {
		const planets = [];
		for (let i = 0; i < output.length(); i++) {
			const planet = output.get_planet(i);
			planets.push({
				name: planet.name(),
				radius: planet.radius(),
				x: planet.x(),
				y: planet.y(),
				z: planet.z(),
			});
		}	
		return planets;
	}


	let output: { name: string; radius:number; x: number; y: number; z: number; }[];
  onMount(async () => {
    await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS

		const universe = Universe.new();

		// universe.add_planet("mars", 6000000, 6000000, 0, 0, -100000, 0, 6e23, 3.389e6);
		universe.add_planet("earth", -6000000, -5000000, 0, 0, 100000, 0, 6e24, 6.371e6);
		universe.add_planet("earth", 6000000, 5000000, 0, 0, -100000, 0, 6e24, 6.371e6);
		// universe.add_planet("moon", 3000000, 0, 0, 0, 1000, 0, 7e22, 1.7e6);



		let i = 0;
		const interval = () => {
				i++;
				if (i % 10 === 0) {
					console.log(universe.as_string());
				}

				universe.tick();
				output = buildArray(universe.render());
				// console.log(output);

				// setTimeout(() => {
					requestAnimationFrame(interval);
				// }, 100);

				// requestAnimationFrame(interval);
		};


		requestAnimationFrame(interval);

  })

</script>

<div>
{#if output}

<div style="position:relative; width:100vmin;height:100vmin">
{#each output as planet}
<div>{planet.name}: ({planet.x}, {planet.y}, {planet.z})</div>
{/each}
<div style={`position:absolute; border-radius:50%; transform:translate(-50%, -50%);top:50%; left:50%;width:${5}px;height:${5}px;background-color:black;`}></div>
{#each output as planet}
	<div data-name={planet.name} style={`position:absolute; transform:translate(calc(-50% + ${planet.x/300000}px), calc(-50% - ${planet.y/300000}px)); border-radius:50%; top:50%; left:50%;width:${planet.radius / 5e5}px;height:${planet.radius / 5e5}px;background-color:#${planet.name === "mars" ? "ff0000" : planet.name === "earth" ? "00ff00" : "aaaaaa"};`}></div>
{/each}
	
</div>

{/if}
</div>

