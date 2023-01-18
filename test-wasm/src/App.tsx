import React from "react";
import "./App.css";
import * as wasm from "poc-test-wasm";

wasm.default();

const canva_ctx = wasm.start();

const draw = () =>  {
  wasm.draw_face(canva_ctx);
}

let i = 50;
let j = 50;

function App() {

  const [text, setText] = React.useState("");
  const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
    setText(event.currentTarget.value);
  };

  const drawline = () =>  {
    wasm.draw_line(canva_ctx, i += 10, j += 10);
  }

  const drawtext = () =>  {
    wasm.draw_text(canva_ctx, text, i += 10, j += 10, 300);
  }

  const drawrect = () =>  {
    wasm.draw_rect(canva_ctx, i += 10, j += 10, 20, 30);
  }

  const setCyan = () =>  {
    wasm.set_color(canva_ctx, "cyan");
  }

  const setRed = () =>  {
    wasm.set_color(canva_ctx, "red");
  }

  const setGreen = () =>  {
    wasm.set_color(canva_ctx, "green");
  }

  return (
    <div className="App">
      <input onChange={handleChange}/>
      <button onClick={draw}>draw face</button>
      <button onClick={drawline}>line</button>
      <button onClick={drawtext}>text</button>
      <button onClick={drawrect}>rect</button>
      <button onClick={setCyan}>cyan</button>
      <button onClick={setRed}>red</button>
      <button onClick={setGreen}>green</button>
      <div>
        <canvas id="canvas">
        </canvas>
      </div>
    </div>
  );
}


export default App;
