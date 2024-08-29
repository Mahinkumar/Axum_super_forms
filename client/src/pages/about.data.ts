import type { RouteDataFunc } from '@solidjs/router';
import { getCookie, setCookie } from 'typescript-cookie'
import { createResource, Resource } from 'solid-js';

let x: number;

function wait<T>(ms: number, data: T): Promise<T> {
  return new Promise((resolve) => setTimeout(resolve, ms, data));
}

function random(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function fetchName(): Promise<string> {
  setCookie('Cookie_from', 'browser');
  x = random(500, 1000);
  return wait(random(500, 1000), 'Solid in '+ x + ' ms' );
}

const AboutData: RouteDataFunc<never, Resource<string>> = () => {
  const [data] = createResource(fetchName);
  return data;
  
};

export default AboutData;
export type AboutDataType = typeof AboutData;
