import React, { useCallback } from "react";
import logo from "./logo.svg";
import "./App.css";
import * as wasm from "poc-test-wasm";

wasm.default();

function App() {
  // useCallback(() => {
    console.log("wasm = ", wasm);
  // }, []);

  return (
    <div className="App">
      {/* <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header> */}
      <button onClick={wasm.display_blue_square}>blue circle</button>
      <button onClick={wasm.display_red_square}>red circle</button>
    </div>
  );
}

export default App;
