<script lang="ts">
    const urlParams = new URLSearchParams(window.location.search);
    const data = {
        app: urlParams.get('app') || 'Unknown'
    };
    let app = data.app == "undefined" ? "Heatmap" : data.app;

    import { invoke } from "@tauri-apps/api/tauri";
    import 'bootstrap/dist/css/bootstrap.min.css';
    import 'bootstrap/dist/js/bootstrap.bundle.min.js';

    import Header from '../../components/Header.svelte';
    import Sidebar from '../../components/Sidebar.svelte';
    import { onMount } from "svelte";

    const NameToId: { [key: string]: string } = {
        "South": "path4325",
        "West": "path4333",
        "East": "path4337",
        "North": "path4341",
        "LS": "path3892",
        "RS": "path3892-9",
        "Mode": "path3892-9-3",
        "Start": "rect3922",
        "Select": "rect3922-8",
        "RightTrigger2": "path4171",
        "LeftTrigger2": "path4171-2",
        "RightTrigger": "path3057",
        "LeftTrigger": "path3057-1"
    };

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
      // get the max value of the buckets
      let max = Math.max(...Object.values(axis.pos_buckets));

      let ctx = node.getContext('2d');
      if (ctx) {
        for (const [key, value] of Object.entries(axis.pos_buckets)) {
          let k = axis.h*+key;
          ctx.beginPath();
          // ctx.arc(32, 32, 32, (k+1)*Math.PI, (k+axis.h+1)*Math.PI);
          ctx.rect((k+1)*32, 32-5, (axis.h+1)*4, 10);
          ctx.fillStyle = `rgba(${value/max*255}, 0, ${(1-value/max)*255}, 0.5)`;
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

            let svg = document.getElementById("heatmap")?.getSVGDocument().getElementById("svg2");
            if (!svg) {
                console.error("SVG not found");
                return;
            }

            // get button with most presses
            let max = Math.max(...stats.presses.map((button) => button.presses));

            for (const button of stats.presses) {
                let id = NameToId[button.name];
                if (!id) {
                    console.error(`Button ${button.name} not found`);
                    continue;
                }

                svg.getElementById(id).style.fill = `rgba(${button.presses/max*255}, 0, ${(1-button.presses/max)*255}, 0.5)`;
            }
        });
    }

    onMount(() => {
        invoke("app_stats", { app: data.app, timeframe }).then((data) => {
            stats = data as AppStats;

            let svg = document.getElementById("heatmap")?.getSVGDocument().getElementById("svg2");
            if (!svg) {
                console.error("SVG not found");
                return;
            }

            // get button with most presses
            let max = Math.max(...stats.presses.map((button) => button.presses));

            for (const button of stats.presses) {
                let id = NameToId[button.name];
                if (!id) {
                    console.error(`Button ${button.name} not found`);
                    continue;
                }

                svg.getElementById(id).style.fill = `rgba(${button.presses/max*255}, 0, ${(1-button.presses/max)*255}, 0.5)`;
            }
        });
    });
</script>

<div class="">
    <Header />
  <div class="container-fluid body">
    <div class="row">
      <Sidebar app="{data.app}" />
      <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
        <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
          <h1 class="h2">{app}</h1>
          <div class="btn-toolbar mb-2 mb-md-0">
            <!-- <div class="btn-group me-2">
              <button type="button" class="btn btn-sm btn-outline-secondary">Share</button>
              <button type="button" class="btn btn-sm btn-outline-secondary">Export</button>
            </div> -->
  
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

        <object id="heatmap" type="image/svg+xml" data="controller.svg" title="controller"></object>
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
  
