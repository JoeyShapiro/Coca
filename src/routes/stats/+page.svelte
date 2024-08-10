<script lang="ts">
    const urlParams = new URLSearchParams(window.location.search);
    const data = {
        app: urlParams.get('app') || 'Unknown'
    };

    import { invoke } from "@tauri-apps/api/tauri";
    import 'bootstrap/dist/css/bootstrap.min.css';
    import 'bootstrap/dist/js/bootstrap.bundle.min.js';

    import Header from '../../components/Header.svelte';
    import Sidebar from '../../components/Sidebar.svelte';
    import { onMount } from "svelte";

    // TODO i could dupe everything, but that would be pointless. this is a summary

    class Button {
        name: string;
        presses: number;

        constructor(name: string, presses: number) {
            this.name = name;
            this.presses = presses;
        }
    }
    
    class Axis {
        name: string;
        pos_buckets: number[];
        h: number;

        constructor(name: string, pos_buckets: number[], h: number) {
            this.name = name;
            this.pos_buckets = pos_buckets;
            this.h = h;
        }
    }

    function onHeatmapAdded(node: HTMLCanvasElement, axis: Axis) {
      let ctx = node.getContext('2d');
      if (ctx) {
        for (const [key, value] of Object.entries(axis.pos_buckets)) {
          let k = axis.h*+key;
          ctx.beginPath();
          ctx.arc(32, 32, 32, (k+1)*Math.PI, (k+axis.h+1)*Math.PI);
          console.log(`'rgba(${value/200*255}, 0, 0, 0.5)'`);
          ctx.fillStyle = `rgba(${value/200*255}, 0, 0, 0.5)`;
          ctx.fill();
        }
      }

      return {
        destroy() {
          // Optional: Clean up code
        }
      };
    };

    class Combo {
        name: string;
        pattern: string;
        presses: number;

        constructor(name: string, pattern: string, presses: number) {
            this.name = name;
            this.pattern = pattern;
            this.presses = presses;
        }
    }

    class AppStats {
        app: string;
        presses: Button[];
        axes: Axis[];
        combos: Combo[];

        constructor(app: string, presses: Button[], axes: Axis[], combos: Combo[]) {
            this.app = app;
            this.presses = presses;
            this.axes = axes;
            this.combos = combos;
        }
    }

    let timeframe: string = "week";
    let stats: AppStats = new AppStats("", [], [], []);

    function changeTime() {
        invoke("app_stats", { app: data.app, timeframe }).then((data) => {
            stats = data as AppStats;
        });
    }

    onMount(() => {
        invoke("app_stats", { app: data.app, timeframe }).then((data) => {
            stats = data as AppStats;
            console.log(stats);
        });
    });
</script>

<div class="">
    <Header />
  <div class="container-fluid body">
    <div class="row">
      <Sidebar />
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
          <h1 class="h2">{data.app}</h1>
          <div class="btn-toolbar mb-2 mb-md-0">
            <div class="btn-group me-2">
                <button type="button" class="btn btn-sm btn-outline-primary">Add Combo</button>
              <button type="button" class="btn btn-sm btn-outline-secondary">Share</button>
              <button type="button" class="btn btn-sm btn-outline-secondary">Export</button>
            </div>
  
            <div class="dropdown">
              <button class="btn btn-sm btn-outline-secondary gap-1 py-2 px-0 px-lg-2 dropdown-toggle align-items-center" type="button" data-bs-toggle="dropdown">
                <svg class="bi"><use xlink:href="#calendar3"></use></svg>
                Past {timeframe}
              </button>
              <ul class="dropdown-menu dropdown-menu-end">
                <li>
                  <button type="button" class="dropdown-item d-flex align-items-center { timeframe === 'day' ? 'active' : '' }" on:click={() => {timeframe = 'day'; changeTime()}}>
                    <svg class="bi me-2 opacity-50"><use href="#sun-fill"></use></svg>
                    Past day
                  </button>
                </li>
                <li>
                  <button type="button" class="dropdown-item d-flex align-items-center { timeframe === 'week' ? 'active' : '' }" on:click={() => {timeframe = 'week'; changeTime()}}>
                    <svg class="bi me-2 opacity-50"><use href="#sun-fill"></use></svg>
                    Past week
                  </button>
                </li>
                <li>
                  <button type="button" class="dropdown-item d-flex align-items-center { timeframe === 'month' ? 'active' : '' }" on:click={() => {timeframe = 'month'; changeTime()}}>
                    <svg class="bi me-2 opacity-50"><use href="#moon-stars-fill"></use></svg>
                    Past month
                  </button>
                </li>
                <li>
                  <button type="button" class="dropdown-item d-flex align-items-center { timeframe === 'year' ? 'active' : '' }" on:click={() => {timeframe = 'year'; changeTime()}}>
                    <svg class="bi me-2 opacity-50"><use href="#circle-half"></use></svg>
                    Past Year
                  </button>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div class="row p-2">
            <h3 class="h2">Presses</h3>
            {#each stats.presses as press}
            <div class="col-md-2 p-2">
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">{press.name}</h5>
                        <p class="card-text">{press.presses}</p>
                    </div>
                </div>
            </div>
            {/each}
        </div>
        <div class="row p-2">
            <h3 class="h2">Combos</h3>
            {#each stats.combos as combo}
            <div class="col-md-2 p-2">
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">{combo.name}</h5>
                        <p class="card-text">{combo.pattern}</p>
                        <p class="card-text">{combo.presses}</p>
                    </div>
                </div>
            </div>
            {/each}
        </div>
        <div class="row p-2">
          <h3 class="h2">Axes</h3>
          {#each stats.axes as axis}
          <div class="col-md-2 p-2">
              <div class="card">
                  <div class="card-body">
                      <h5 class="card-title">{axis.name}</h5>
                      <canvas use:onHeatmapAdded={axis} width="64" height="64"></canvas>
                  </div>
              </div>
          </div>
          {/each}
      </div>
      </main>
    </div>
  </div>
  
  </div>
  
  <style>
    .bi {
    width: 1em;
    height: 1em;
  }
  
  .body {
    padding-top: 50px; /* Adjust this value based on your navbar height */
  }
  
  main {
      margin-left: 250px; /* Should match the sidebar width */
  }
  </style>
  
