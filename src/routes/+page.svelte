<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import 'bootstrap/dist/css/bootstrap.min.css';
  import 'bootstrap/dist/js/bootstrap.bundle.min.js';
  import { onMount } from 'svelte';

  import { Line } from 'svelte-chartjs';

  import Header from '../components/Header.svelte';
  import Sidebar from '../components/Sidebar.svelte';

  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale,
    type ChartData,
  } from 'chart.js';

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    LinearScale,
    PointElement,
    CategoryScale
  );

  class Application {
    name: string;
    controller: string;
    presses: number;
    combos: number;

    constructor(name: string, controller: string, presses: number, combos: number) {
      this.name = name;
      this.controller = controller;
      this.presses = presses;
      this.combos = combos;
    }
  }

  class Point {
    data: number;
    label: string;

    constructor(data: number, label: string) {
      this.data = data;
      this.label = label;
    }
  }

  let apps: Application[] = [];
  let points: Point[] = [];
  let timeframe: string = "week";
  let graphLoading: boolean = false;
  let appsLoading: boolean = false;

  let chartData: ChartData<'line', number[], unknown> = {
    labels: points.map((point) => point.label),
    datasets: [
      {
        label: 'Total presses',
        fill: true,
        tension: 0.3,
        backgroundColor: 'rgba(225, 204,230, .3)',
        borderColor: 'rgb(205, 130, 158)',
        borderCapStyle: 'butt',
        borderDash: [],
        borderDashOffset: 0.0,
        borderJoinStyle: 'miter',
        pointBorderColor: 'rgb(205, 130,1 58)',
        pointBackgroundColor: 'rgb(255, 255, 255)',
        pointBorderWidth: 10,
        pointHoverRadius: 5,
        pointHoverBackgroundColor: 'rgb(0, 0, 0)',
        pointHoverBorderColor: 'rgba(220, 220, 220,1)',
        pointHoverBorderWidth: 2,
        pointRadius: 1,
        pointHitRadius: 10,
        data: points.map((point) => point.data),
      },
    ],
  };

  function changeTime() {
    graphLoading = true;
    appsLoading = true;

    invoke("graph", { timeframe }).then((data) => {
      graphLoading = false;
      points = data as Point[];
      chartData.labels = points.map((point) => point.label);
      chartData.datasets[0].data = points.map((point) => point.data);
    }).catch((e) => {
      console.error(e);
    });

    invoke("applications", { timeframe }).then((data) => {
      appsLoading = false;
      apps = data as Application[];
      apps.sort((a, b) => b.presses - a.presses);
    }).catch((e) => {
      console.error(e);
    })
  }

  onMount(async () => {
    apps = await invoke("applications", { timeframe });
    // sort by presses
    apps.sort((a, b) => b.presses - a.presses);

    // get points
    points = await invoke("graph", { timeframe });

    chartData.labels = points.map((point) => point.label);
    chartData.datasets[0].data = points.map((point) => point.data);
  });

</script>

<div class="">
  <Header />
<div class="container-fluid body">
  <div class="row">
    <Sidebar app={undefined} />
    <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
      <div class="d-flex justify-content-between flex-wrap flex-md-nowrap align-items-center pt-3 pb-2 mb-3 border-bottom">
        <h1 class="h2">Overview</h1>
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

      {#if graphLoading}
        <p>Crunching the numbers just for you :)</p>
      {:else}
        <Line data={chartData} options={{ responsive: true }} />
      {/if}

      <h2>Applications</h2>
      {#if appsLoading}
        <p>Crunching the numbers just for you :)</p>
      {:else}
        <div class="table-responsive small">
          <table class="table table-striped table-sm">
            <thead>
              <tr>
                <th scope="col">#</th>
                <th scope="col">Application</th>
                <th scope="col">Presses</th>
                <th scope="col">Combos</th>
              </tr>
            </thead>
            <tbody>
              {#each apps as app, i}
                <tr>
                  <td>{i + 1}</td>
                  <td><a class="nav-link" href="/stats?app={app.name}">{app.name}</a></td>
                  <td>{app.presses}</td>
                  <td>{app.combos}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      <h2>Heatmap</h2>
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
