import type { Component } from "solid-js";
import { Link, useRoutes, useLocation } from "@solidjs/router";

import { routes } from "./routes";

const App: Component = () => {
  const location = useLocation();
  const Route = useRoutes(routes);
  return (
    <>
      {location.pathname=="/login" ? '':<Nav/>}
      <main>
        <Route />
      </main>
    </>
  );
};

function Nav() {
  return (
    <nav class="bg-gray-200 text-gray-900 px-4">
      <ul class="flex items-center">
        <li class="py-2 px-4 pr-12">
          <h1 class="text-bold text-xl">Axum Super Forms</h1>
        </li>
        <li class="py-2 px-4">
          <Link href="/" class="no-underline hover:underline">
            Home
          </Link>
        </li>
        <li class="py-2 px-4">
          <Link href="/forms" class="no-underline hover:underline">
            forms
          </Link>
        </li>
        <li class="text-sm flex items-center space-x-1 ml-auto">
          <span>URL:</span>
          <input
            class="w-75px p-1 bg-white text-sm rounded-lg"
            type="text"
            readOnly
            value={location.pathname}
          />
        </li>
      </ul>
    </nav>
  );
}

export default App;
