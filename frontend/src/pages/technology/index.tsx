import solidLogo from "./assets/solid.svg";
import viteLogo from "./assets/vite.svg";
import "./index.css";

function Technology() {
  return (
    <>
      <h1>This app was built with the below technologies</h1>
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
      <p class="read-the-docs">
        Click on the logos to learn more
      </p>
    </>
  );
}

export default Technology;
