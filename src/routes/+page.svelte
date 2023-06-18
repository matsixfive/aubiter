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
  let setSpeed = (speed:number) => {};
	let addPlanet = () => {};
	let speed = 100;
	let tps = 0

	let framerate = 60; // temp inital value
	let displayFramerate = framerate;

  onMount(async () => {
    await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS

    const universe = Universe.new();
		universe.set_speed(speed);

		// universe.add_planet("earth", Vector3.new(0, 0, 0), Vector3.new(0, 0, 0), 5.972e21, 3.389e6);
		// universe.add_planet("moon", Vector3.new(0, 3844000, 0), Vector3.new(352.2, 0, 0), 7.34767309e19, 3.389e6);
		universe.add_planet("earth", Vector3.new(804000, 0, 0), Vector3.new(0, -300, 0), 5.972e21, 100000);
		// universe.add_planet("earth1", Vector3.new(0, 804000, 0), Vector3.new(800, 0, 0), 5.972e21, 100000);
		universe.add_planet("mars", Vector3.new(-804000, 0, 0), Vector3.new(0, 300, 0), 5.972e21, 100000);
		// universe.add_planet("mars1", Vector3.new(0, -804000, 0), Vector3.new(-800, 0, 0), 5.972e21, 100000);
		// universe.add_planet("moon", Vector3.new(70000, 384400, 0), Vector3.new(1182, 100, 0), 6.39e13, 100000);

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
		setSpeed = (speed: number) => {
			if (speed < 0) return;
      universe.set_speed(speed);
			tps = speed * framerate;
		};
		addPlanet = () => {
			universe.add_planet(Date.now().toString(), Vector3.new(0, 384400, 0), Vector3.new(0, 0, 0), 6.39e13, 3.389e6);
		};
	

		let lastDate = performance.now();
    let loop = () => {
			universe.tick();

			const render = universe.render();
      output = buildArray(render);

			framerate = 1000 / (performance.now() - lastDate);
			lastDate = performance.now();

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

	let zoom = 10000;
</script>

<div>
  {#if output}
    <div style="position:relative; width:100vmin;height:100vmin">
			{Math.round(displayFramerate)}fps
			<button on:click={addPlanet}>Add Planet</button>
      {#each output as planet}
        <div>{planet.name}: ({Math.round(Math.pow((Math.pow(planet.velocity.x(), 2) + Math.pow(planet.velocity.y(), 2)), 0.5))})</div>
      {/each}
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
            planet.position.x() / zoom
          }px), calc(-50% - ${
            planet.position.y() / zoom
          }px)); border-radius:50%; top:50%; left:50%;width:${
            planet.radius / (zoom / 2)
          }px;height:${planet.radius / (zoom / 2)}px;background-color:#${
            colors.hasOwnProperty(planet.name) ? colors[planet.name] : "aaaaaa"
          };`}
        />
      {/each}
    </div>
  {/if}
</div>
