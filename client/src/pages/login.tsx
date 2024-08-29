import { createMemo, createSignal } from "solid-js";

export default function Home() {
  const [is_auth, setis_auth] = createSignal(false);
  const handleButtonClick = () => setis_auth(!is_auth());

  return (
    <>
      <div class="bg-gray-100 p-4 flex flex-col justify-center items-center w-screen h-screen">
        <div class="w-96 h-80 bg-white rounded-lg shadow p-6 flex flex-col justify-around">
          <h1 class="text-3xl text-center">Axum Login</h1>
          <button onClick={handleButtonClick}>
            {is_auth() ? "Via Auth" : "Via KeyCode"}
          </button>
          {is_auth() ? <Auth_form /> : <Key_form />}
        </div>
      </div>
    </>
  );
}

function Auth_form() {
  return (
    <>
      <form class="space-y-4 flex flex-col justify-around items-center">
        <input
          type="email"
          placeholder="Email Address"
          class="block w-[80%] px-4 py-2 text-gray-700 border 
rounded-lg appearance-none focus:outline-none focus:border-blue-300 
bg-gray-100"
        />
        <input
          type="password"
          placeholder="Password"
          class="block w-[80%] px-4 py-2 text-gray-700 border 
rounded-lg appearance-none focus:outline-none focus:border-blue-300 
bg-gray-100"
        />
        <button
          class="w-48 h-12 text-white transition duration-200 ease-out bg-green-500 hover:bg-green-600 rounded-lg self-center"
          type="submit"
        >
          Login
        </button>
      </form>
    </>
  );
}

function Key_form() {
  return (
    <>
      <form class="space-y-4 flex flex-col justify-around items-center">
        <input
          type="password"
          placeholder="Key"
          class="block w-[80%] px-4 py-2 text-gray-700 border 
rounded-lg appearance-none focus:outline-none focus:border-blue-300 
bg-gray-100"
        />
        <button
          class="w-48 h-12 text-white transition duration-200 ease-out bg-green-500 hover:bg-green-600 rounded-lg self-center"
          type="submit"
        >
          Login
        </button>
      </form>
    </>
  )
}
