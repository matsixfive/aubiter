<script lang="ts">
  import init, { Universe, Output, Vector3 } from "gravity";
  // we need onMount to run init
  import { onMount } from "svelte";

  const buildArray = (output: Output) => {
    const planets = [];
    for (let i = 0; i < output.length(); i++) {
      const planet = output.get_planet(i);
      planets.push({
        name: planet.name(),
        radius: planet.radius(),
				velocity: planet.velocity(),
        position: planet.position(),
      });
    }
    return planets;
  };

  const colors = {
    earth: "00ff00",
    mars: "ff0000",
    moon: "0a0a0a",
    sun: "ffff00",
  };

  let output: {
    name: string;
    radius: number;
		velocity: Vector3;
		position: Vector3;
  }[];

  let speedUp = () => {};
  let speedDown = () => {};
	let addPlanet = () => {};
	let speed = 1;
	let tps = 0

	let framerate = 60;

  onMount(async () => {
    await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS

    const universe = Universe.new();
		universe.set_speed(speed);

		universe.add_planet("earth", Vector3.new(0, 0, 0), Vector3.new(0, 0, 0), 5.972e21, 3.389e6);
		universe.add_planet("moon", Vector3.new(0, 3844000, 0), Vector3.new(352.2, 0, 0), 7.34767309e19, 3.389e6);

    speedUp = () => {
      universe.set_speed(speed *= 1.25);
		};
    speedDown = () => {
			if (speed > 0) {
				universe.set_speed(speed /= 1.25);
			}
		};
		addPlanet = () => {
			universe.add_planet("new", Vector3.new(0, 3844000, 0), Vector3.new(1022, 0, 0), 6.39e23, 3.389e6);
		};


		let lastDate = performance.now();
    let loop = () => {
			universe.tick();

      output = buildArray(universe.render());

			framerate = 1000 / (performance.now() - lastDate);
			lastDate = performance.now();

      requestAnimationFrame(loop);
    };

    requestAnimationFrame(loop);

		let interval = setInterval(() => {
			tps = speed * framerate;
		}, 1000);

		return () => {
			universe.free();
			clearInterval(interval);
			loop = () => {};
		};
  });
</script>

<div>
  {#if output}
    <div style="position:relative; width:100vmin;height:100vmin">
			{Math.round(framerate)}fps
			<button on:click={addPlanet}>Add Planet</button>
      {#each output as planet}
        <div>{planet.name}: ({Math.round(Math.pow((Math.pow(planet.velocity.x(), 2) + Math.pow(planet.velocity.y(), 2)), 0.5))})</div>
      {/each}
			<button on:click={speedDown}>{"<<"}</button>
				{Math.round(speed * 100) / 100}x
			<button on:click={speedUp}>{">>"}</button>
			{Math.ceil(tps)}tps

      <div
        style={`position:absolute; border-radius:50%; transform:translate(-50%, -50%);top:50%; left:50%;width:${2}px;height:${2}px;background-color:black;`}
      />
      {#each output as planet}
        <div
          data-name={planet.name}
          style={`position:absolute; transform:translate(calc(-50% + ${
            planet.position.x() / 20000
          }px), calc(-50% - ${
            planet.position.y() / 20000
          }px)); border-radius:50%; top:50%; left:50%;width:${
            planet.radius / 5e5
          }px;height:${planet.radius / 5e5}px;background-color:#${
            colors.hasOwnProperty(planet.name) ? colors[planet.name] : "aaaaaa"
          };`}
        />
      {/each}
    </div>
  {/if}
</div>
