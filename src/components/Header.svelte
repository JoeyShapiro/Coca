<script>
    import { page } from '$app/stores';
</script>

<nav class="p-2 border-bottom navbar-expand-lg bg-light fixed-top">
    <div class="container">
        <div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
            <ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
                <li><a href="/" class="nav-link px-2 { $page.url.pathname === '/' ? 'link-secondary' : 'link-body-emphasis' }">Dashboard</a></li>
                <li><a href="/applications" class="nav-link px-2 { $page.url.pathname === '/applications' ? 'link-secondary' : 'link-body-emphasis' }">Applications</a></li>
                <li><a href="/playground" class="nav-link px-2 { $page.url.pathname === '/playground' ? 'link-secondary' : 'link-body-emphasis' }">Playground</a></li>
            </ul>

            <form class="col-12 col-lg-auto mb-3 mb-lg-0 me-lg-3" role="search">
                <input type="search" class="form-control" placeholder="Search..." aria-label="Search">
            </form>
        </div>
    </div>
</nav>
