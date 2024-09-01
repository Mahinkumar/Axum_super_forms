import { Component, createEffect, Suspense } from 'solid-js';
import { useRouteData } from '@solidjs/router';

export default function About() {

  return (
    <section class="bg-pink-100 text-gray-700 p-8">
      <h1 class="text-2xl font-bold">Forms</h1>
      <p class="mt-4">This is a test forms page</p>
    </section>
  );
}
