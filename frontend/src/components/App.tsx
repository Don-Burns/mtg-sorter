import { createSignal } from "solid-js";
import solidLogo from "../assets/solid.svg";
import viteLogo from "../assets/vite.svg";
import "../components/App.css";

function App() {
  const [count, setCount] = createSignal(0);

  return (
    <>
      <div>
        <form
          action="/upload"
          method="post"
          enctype="multipart/form-data"
          target="_top"
        >
          <input type="file" id="file_upload" name="file_upload" />
          <input
            type="submit"
            value="Upload"
            formtarget="_top"
          />
        </form>
      </div>

      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} class="logo" alt="Vite logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={solidLogo} class="logo solid" alt="Solid logo" />
        </a>
        <a href="https://github.com/tokio-rs/axum" target="_blank">
          <img
            src="https://avatars.githubusercontent.com/u/20248544"
            class="logo axum"
            alt="Axum logo"
          />
        </a>
      </div>
      <h1>Vite + Solid + Axum</h1>
      <div class="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count()}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p class="read-the-docs">
        Click on the Vite and Solid logos to learn more
      </p>
    </>
  );
}

export default App;
