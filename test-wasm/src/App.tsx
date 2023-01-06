import React, { useCallback } from "react";
import logo from "./logo.svg";
import "./App.css";
import * as wasm from "poc-test-wasm";

wasm.default();

const display = () => {
  wasm.donut_display('elyes');
}

function App() {

  return (
    <div className="App">
      <button onClick={wasm.display_blue_square}>blue circle</button>
      <button onClick={display}>red circle</button>
      <button onClick={wasm.display_rocket}>rocket pls</button>
    </div>
  );
}


export default App;
